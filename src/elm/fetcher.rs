use crate::elm::PackageInfo;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value as JsonValue;

// Custom deserializer for comment fields that might be either a string or an array
fn deserialize_comment<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = JsonValue::deserialize(deserializer)?;
    match value {
        JsonValue::String(s) => Ok(s),
        JsonValue::Array(arr) if arr.is_empty() => Ok(String::new()),
        _ => Ok(String::new()), // Default to empty string for any other case
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    #[serde(deserialize_with = "deserialize_comment")]
    pub comment: String,
    pub unions: Vec<Union>,
    pub aliases: Vec<Alias>,
    pub values: Vec<Value>,
    pub binops: Vec<Binop>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alias {
    pub name: String,
    #[serde(deserialize_with = "deserialize_comment")]
    pub comment: String,
    pub args: Vec<String>,
    #[serde(rename = "type")]
    pub type_annotation: String,
}

// Custom deserializer for union cases which are [name, [types...]]
//
// The Elm package API returns union cases in a nested array format:
// [[constructorName, [typeArg1, typeArg2, ...]], ...]
//
// For example, the Review.Fix.FixResult union has cases like:
// [["Successful", ["String.String"]], ["Errored", ["Review.Fix.Problem"]]]
//
// This caused a parsing error with packages like elm-review because we were
// expecting Vec<Vec<String>> (flat structure) but the API returns a nested structure
// where the second element is itself an array of type arguments.
//
// This deserializer flattens the structure into Vec<Vec<String>> where:
// - First element is the constructor name
// - Remaining elements are the type arguments
fn deserialize_cases<'de, D>(deserializer: D) -> Result<Vec<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = JsonValue::deserialize(deserializer)?;
    match value {
        JsonValue::Array(cases) => {
            let mut result = Vec::new();
            for case in cases {
                if let JsonValue::Array(case_arr) = case {
                    if case_arr.len() >= 2 {
                        // First element should be the constructor name
                        if let JsonValue::String(name) = &case_arr[0] {
                            let mut case_items = vec![name.clone()];

                            // Second element should be an array of types
                            if let JsonValue::Array(types) = &case_arr[1] {
                                for typ in types {
                                    if let JsonValue::String(type_str) = typ {
                                        case_items.push(type_str.clone());
                                    }
                                }
                            }
                            result.push(case_items);
                        }
                    }
                }
            }
            Ok(result)
        }
        _ => Ok(Vec::new()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Union {
    pub name: String,
    #[serde(deserialize_with = "deserialize_comment")]
    pub comment: String,
    pub args: Vec<String>,
    #[serde(deserialize_with = "deserialize_cases")]
    pub cases: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub name: String,
    #[serde(rename = "type")]
    pub type_annotation: String,
    #[serde(deserialize_with = "deserialize_comment")]
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binop {
    pub name: String,
    #[serde(rename = "type")]
    pub type_annotation: String,
    #[serde(deserialize_with = "deserialize_comment")]
    pub comment: String,
    pub associativity: String,
    pub precedence: i32,
}

pub async fn fetch_readme(package: &PackageInfo) -> Result<String, String> {
    let url = format!(
        "https://package.elm-lang.org/packages/{}/{}/{}/README.md",
        package.author, package.name, package.version
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch README: {e}"))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch README: HTTP {}",
            response.status()
        ));
    }

    response
        .text()
        .await
        .map_err(|e| format!("Failed to read README content: {e}"))
}

pub async fn fetch_docs(package: &PackageInfo) -> Result<Vec<Module>, String> {
    let url = format!(
        "https://package.elm-lang.org/packages/{}/{}/{}/docs.json",
        package.author, package.name, package.version
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch docs: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch docs: HTTP {}", response.status()));
    }

    let modules: Vec<Module> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse docs JSON: {e}"))?;

    Ok(modules)
}
