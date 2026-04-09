use crate::config::AppConfig;
use crate::storage::r2::R2Client;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub s3: R2Client,
    pub config: Arc<AppConfig>,
    pub oauth_client: Arc<oauth2::basic::BasicClient>,
}
