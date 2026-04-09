use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, State},
    http::header,
    response::Response,
};
use tokio_util::io::ReaderStream;

use crate::{db, error::AppError, state::AppState};

fn blob_lookup_error(name: &str, version: &str, error: &str) -> AppError {
    if is_missing_blob_error(error) {
        AppError::NotFound
    } else {
        AppError::Internal(format!(
            "Failed to fetch package blob for {}@{}: {}",
            name, version, error
        ))
    }
}

fn is_missing_blob_error(error: &str) -> bool {
    let error = error.to_ascii_lowercase();
    error.contains("not found")
        || error.contains("no such key")
        || error.contains("nosuchkey")
        || error.contains("404")
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path((owner, package, version)): Path<(String, String, String)>,
) -> Result<Response, AppError> {
    let name = format!("{}/{}", owner, package);
    // Get version record (for sha256 key)
    let ver = db::packages::get_version(&state.pool, &name, &version)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or(AppError::NotFound)?;

    // Fetch from R2 first so missing blob state is visible and counters stay honest.
    let s3_resp = state
        .s3
        .get_object(&ver.sha256)
        .await
        .map_err(|e| blob_lookup_error(&name, &version, &e))?;

    db::packages::increment_download(&state.pool, &name, &version)
        .await
        .map_err(|e| {
            AppError::Internal(format!(
                "Failed to record download for {}@{}: {}",
                name, version, e
            ))
        })?;

    let stream = s3_resp.body.into_async_read();
    let body = Body::from_stream(ReaderStream::new(stream));

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!(
                "attachment; filename=\"{}-{}.tar.gz\"",
                name.replace('/', "-"),
                version
            ),
        )
        .body(body)
        .unwrap())
}

#[cfg(test)]
mod tests {
    use super::is_missing_blob_error;

    #[test]
    fn missing_blob_error_strings_are_detected() {
        assert!(is_missing_blob_error("R2 get_object error: NoSuchKey"));
        assert!(is_missing_blob_error("R2 get_object error: not found"));
        assert!(is_missing_blob_error("R2 get_object error: HTTP 404"));
    }

    #[test]
    fn non_missing_blob_errors_stay_visible() {
        assert!(!is_missing_blob_error(
            "R2 get_object error: timeout while contacting bucket"
        ));
        assert!(!is_missing_blob_error("R2 get_object error: access denied"));
    }
}
