//! Parse KML files from flightlog.org

use anyhow::anyhow;
use regex::Regex;
use tracing::info;
use std::fs;

/// Get Flightlog URLs from a KML file at `path`.
pub async fn get_urls(path: &str) -> Result<Vec<String>, anyhow::Error> {
    let mut out = Vec::new();
    let contents = fs::read_to_string(path)?;
    let re = Regex::new(r"https:\/\/flightlog\.org\/fl\.html\?.+start_id=\d+&quot;&gt;")?;
    let caps = re.captures_iter(&contents);

    for cap in caps {
        let url = cap
            .get(0)
            .ok_or(anyhow!("regex failed"))?
            .as_str()
            .replace("&amp;amp;", "&")
            .replace("&quot;&gt;", "");

        out.push(url);
    }

    info!("Found {} URLs from KML file.", out.len());

    Ok(out)
}
