use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEntry {
    pub name: String,
    pub summary: String,
    pub license: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub name: String,
    pub summary: String,
    pub license: String,
    pub version: String,
    pub score: u32,
}

const SEARCH_URL: &str = "https://package.elm-lang.org/search.json";

/// Fetch the search index from package.elm-lang.org
pub fn fetch_search_index() -> Result<Vec<SearchEntry>, String> {
    let client = reqwest::blocking::Client::builder()
        .gzip(true)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(SEARCH_URL)
        .send()
        .map_err(|e| format!("Failed to fetch search index: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch search index: HTTP {}",
            response.status()
        ));
    }

    let entries: Vec<SearchEntry> = response
        .json()
        .map_err(|e| format!("Failed to parse search index: {}", e))?;

    Ok(entries)
}

/// Perform fuzzy search on package name and summary
/// If exclude_packages is provided, those packages will be filtered out
pub fn fuzzy_search(
    query: &str,
    entries: &[SearchEntry],
    exclude_packages: Option<&HashSet<String>>,
    limit: usize,
) -> Vec<SearchResult> {
    let mut matcher = Matcher::new(Config::DEFAULT);
    let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);

    let mut results: Vec<(SearchEntry, u32)> = entries
        .iter()
        .filter_map(|entry| {
            // Filter out excluded packages if specified
            if let Some(excluded) = exclude_packages {
                if excluded.contains(&entry.name) {
                    return None;
                }
            }

            // Create searchable text: "name summary"
            let searchable = format!("{} {}", entry.name, entry.summary);
            let mut buf = Vec::new();
            let haystack = Utf32Str::new(&searchable, &mut buf);

            pattern
                .score(haystack, &mut matcher)
                .map(|score| (entry.clone(), score))
        })
        .collect();

    // Sort by score descending
    results.sort_by(|a, b| b.1.cmp(&a.1));

    // Take top N results and convert to SearchResult
    results
        .into_iter()
        .take(limit)
        .map(|(entry, score)| SearchResult {
            name: entry.name,
            summary: entry.summary,
            license: entry.license,
            version: entry.version,
            score,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_search() {
        let entries = vec![
            SearchEntry {
                name: "elm/json".to_string(),
                summary: "Encode and decode JSON values".to_string(),
                license: "BSD-3-Clause".to_string(),
                version: "1.0.0".to_string(),
            },
            SearchEntry {
                name: "elm/html".to_string(),
                summary: "Fast HTML, rendered with virtual DOM diffing".to_string(),
                license: "BSD-3-Clause".to_string(),
                version: "1.0.0".to_string(),
            },
        ];

        let results = fuzzy_search("json", &entries, None, 10);
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "elm/json");
    }

    #[test]
    fn test_fuzzy_search_with_exclusions() {
        let entries = vec![
            SearchEntry {
                name: "elm/json".to_string(),
                summary: "Encode and decode JSON values".to_string(),
                license: "BSD-3-Clause".to_string(),
                version: "1.0.0".to_string(),
            },
            SearchEntry {
                name: "NoRedInk/elm-json-decode-pipeline".to_string(),
                summary: "Use pipelines to build JSON decoders".to_string(),
                license: "BSD-3-Clause".to_string(),
                version: "1.0.0".to_string(),
            },
        ];

        let mut excluded = HashSet::new();
        excluded.insert("elm/json".to_string());

        let results = fuzzy_search("json", &entries, Some(&excluded), 10);
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "NoRedInk/elm-json-decode-pipeline");
    }
}
