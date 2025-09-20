use crate::elm::PackageInfo;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value as JsonValue;
use std::fs;
use std::path::PathBuf;

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

pub fn fetch_readme(package: &PackageInfo) -> Result<String, String> {
    let package_path = get_package_path(package)?;
    let readme_path = package_path.join("README.md");

    if !readme_path.exists() {
        return Err(format!(
            "README.md not found for package {}/{} version {}. Make sure the package is installed locally.",
            package.author, package.name, package.version
        ));
    }

    fs::read_to_string(&readme_path).map_err(|e| format!("Failed to read README.md: {e}"))
}

pub fn fetch_docs(package: &PackageInfo) -> Result<Vec<Module>, String> {
    let package_path = get_package_path(package)?;
    let docs_path = package_path.join("docs.json");

    if !docs_path.exists() {
        return Err(format!(
            "docs.json not found for package {}/{} version {}. Make sure the package is installed locally.",
            package.author, package.name, package.version
        ));
    }

    let docs_content =
        fs::read_to_string(&docs_path).map_err(|e| format!("Failed to read docs.json: {e}"))?;

    let modules: Vec<Module> = serde_json::from_str(&docs_content)
        .map_err(|e| format!("Failed to parse docs JSON: {e}"))?;

    Ok(modules)
}

fn get_package_path(package: &PackageInfo) -> Result<PathBuf, String> {
    let home_dir =
        std::env::var("HOME").map_err(|_| "Could not determine HOME directory".to_string())?;

    let package_path = PathBuf::from(home_dir)
        .join(".elm")
        .join("0.19.1")
        .join("packages")
        .join(&package.author)
        .join(&package.name)
        .join(&package.version);

    if !package_path.exists() {
        return Err(format!(
            "Package {}/{} version {} not found locally at {}. Make sure it's installed by running 'elm install' in an Elm project that uses this package.",
            package.author, package.name, package.version,
            package_path.display()
        ));
    }

    Ok(package_path)
}
