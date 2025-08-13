use crate::elm::PackageInfo;
use serde_json::Value;
use std::fs;

pub fn read_elm_json(path: &str) -> Result<Value, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read elm.json: {e}"))?;

    serde_json::from_str(&content).map_err(|e| format!("Failed to parse elm.json: {e}"))
}

pub fn find_package(elm_json: &Value, package_name: &str) -> Option<PackageInfo> {
    // Check direct dependencies first
    if let Some(deps) = elm_json.get("dependencies").and_then(|d| d.get("direct")) {
        if let Some(deps_obj) = deps.as_object() {
            if let Some(version) = deps_obj.get(package_name) {
                if let Some(version_str) = version.as_str() {
                    return PackageInfo::from_full_name(package_name, version_str);
                }
            }
        }
    }

    // Then check indirect dependencies
    if let Some(deps) = elm_json.get("dependencies").and_then(|d| d.get("indirect")) {
        if let Some(deps_obj) = deps.as_object() {
            if let Some(version) = deps_obj.get(package_name) {
                if let Some(version_str) = version.as_str() {
                    return PackageInfo::from_full_name(package_name, version_str);
                }
            }
        }
    }

    None
}

pub fn get_direct_packages(elm_json: &Value) -> Vec<PackageInfo> {
    let mut packages = Vec::new();

    if let Some(deps) = elm_json.get("dependencies").and_then(|d| d.get("direct")) {
        if let Some(deps_obj) = deps.as_object() {
            for (name, version) in deps_obj {
                if let Some(version_str) = version.as_str() {
                    if let Some(package) = PackageInfo::from_full_name(name, version_str) {
                        packages.push(package);
                    }
                }
            }
        }
    }

    packages
}

pub fn get_indirect_packages(elm_json: &Value) -> Vec<PackageInfo> {
    let mut packages = Vec::new();

    if let Some(deps) = elm_json.get("dependencies").and_then(|d| d.get("indirect")) {
        if let Some(deps_obj) = deps.as_object() {
            for (name, version) in deps_obj {
                if let Some(version_str) = version.as_str() {
                    if let Some(package) = PackageInfo::from_full_name(name, version_str) {
                        packages.push(package);
                    }
                }
            }
        }
    }

    packages
}
