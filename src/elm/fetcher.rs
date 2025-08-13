use crate::elm::PackageInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub comment: String,
    pub unions: Vec<Union>,
    pub aliases: Vec<Alias>,
    pub values: Vec<Value>,
    pub binops: Vec<Binop>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alias {
    pub name: String,
    pub comment: String,
    pub args: Vec<String>,
    #[serde(rename = "type")]
    pub type_annotation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Union {
    pub name: String,
    pub comment: String,
    pub args: Vec<String>,
    pub cases: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub name: String,
    #[serde(rename = "type")]
    pub type_annotation: String,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binop {
    pub name: String,
    #[serde(rename = "type")]
    pub type_annotation: String,
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
        .map_err(|e| format!("Failed to fetch README: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch README: HTTP {}",
            response.status()
        ));
    }

    response
        .text()
        .await
        .map_err(|e| format!("Failed to read README content: {}", e))
}

pub async fn fetch_docs(package: &PackageInfo) -> Result<Vec<Module>, String> {
    let url = format!(
        "https://package.elm-lang.org/packages/{}/{}/{}/docs.json",
        package.author, package.name, package.version
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch docs: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch docs: HTTP {}", response.status()));
    }

    let modules: Vec<Module> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse docs JSON: {}", e))?;

    Ok(modules)
}
