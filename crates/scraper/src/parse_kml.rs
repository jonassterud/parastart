#![deny(missing_docs)]

//! Parse KML files from flightlog.org.

use anyhow::anyhow;
use regex::Regex;
use std::fs;
use tracing::info;

/// Get Flightlog URLs from KML file.
/// 
/// # Arguments
/// 
/// * `path` - A filepath to a KML file from flightlog.org.
/// 
/// # Errors
/// 
/// This function will return an error if Regex or parsing fails.
/// 
/// # Returns
/// 
/// A list of URLs.
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
