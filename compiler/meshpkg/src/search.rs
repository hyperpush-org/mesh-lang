use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct SearchResult {
    name: String,
    version: String,
    description: String,
}

pub fn run(query: &str, registry: &str, json_mode: bool) -> Result<(), String> {
    let results = crate::publish::with_spinner(
        &format!("Searching for \"{}\"...", query),
        json_mode,
        || fetch_results(query, registry),
    )?;

    if results.is_empty() {
        if json_mode {
            println!("[]");
        } else {
            println!("No packages found for \"{}\"", query);
        }
        return Ok(());
    }

    if json_mode {
        let json = serde_json::to_string(&results.iter().map(|r| {
            serde_json::json!({"name": r.name, "version": r.version, "description": r.description})
        }).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".to_string());
        println!("{}", json);
    } else {
        print_search_table(&results);
    }

    Ok(())
}

fn fetch_results(query: &str, registry: &str) -> Result<Vec<SearchResult>, String> {
    let url = format!("{}/api/v1/packages?search={}", registry, query);
    let agent = ureq::Agent::new_with_defaults();
    let mut response = agent
        .get(&url)
        .call()
        .map_err(|e| format!("Failed to search registry: {}", e))?;
    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("Failed to read search response: {}", e))?;
    serde_json::from_str::<Vec<SearchResult>>(&body)
        .map_err(|e| format!("Failed to parse search results: {}", e))
}

fn print_search_table(results: &[SearchResult]) {
    let max_name = results
        .iter()
        .map(|r| r.name.len())
        .max()
        .unwrap_or(4)
        .max(4);
    let max_ver = results
        .iter()
        .map(|r| r.version.len())
        .max()
        .unwrap_or(7)
        .max(7);

    println!(
        "{:<w_name$}  {:<w_ver$}  DESCRIPTION",
        "NAME",
        "VERSION",
        w_name = max_name,
        w_ver = max_ver
    );
    println!("{}", "-".repeat(max_name + max_ver + 16));
    for r in results {
        println!(
            "{:<w_name$}  {:<w_ver$}  {}",
            r.name,
            r.version,
            r.description,
            w_name = max_name,
            w_ver = max_ver
        );
    }
}
