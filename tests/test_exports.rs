use elm_package_mcp_server::mcp::tools::{
    get_export_docs, get_exports, GetExportDocsRequest, GetExportsRequest,
};

#[tokio::test]
async fn test_get_exports_without_comments() {
    // Test getting exports for elm/core List module
    let request = GetExportsRequest {
        author: "elm".to_string(),
        name: "core".to_string(),
        version: "1.0.5".to_string(),
        module: Some("List".to_string()),
    };

    let result = get_exports(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(!response.is_error);
    assert_eq!(response.content.len(), 1);

    // Parse the JSON response
    if let Some(elm_package_mcp_server::mcp::types::CallToolResultContent::Text { text }) =
        response.content.first()
    {
        let json_result: serde_json::Value = serde_json::from_str(text).unwrap();

        // Verify structure
        assert_eq!(json_result["author"], "elm");
        assert_eq!(json_result["name"], "core");
        assert_eq!(json_result["version"], "1.0.5");

        // Verify we have modules
        assert!(json_result["modules"].is_array());
        let modules = json_result["modules"].as_array().unwrap();
        assert_eq!(modules.len(), 1);

        let list_module = &modules[0];
        assert_eq!(list_module["name"], "List");

        // Verify we have values without comments
        assert!(list_module["values"].is_array());
        let values = list_module["values"].as_array().unwrap();
        assert!(!values.is_empty());

        // Check that values have name and type but no comment
        let first_value = &values[0];
        assert!(first_value["name"].is_string());
        assert!(first_value["type"].is_string());
        assert!(first_value.get("comment").is_none());
    } else {
        panic!("Expected text content");
    }
}

#[tokio::test]
async fn test_get_export_docs() {
    // Test getting comment for List.map
    let request = GetExportDocsRequest {
        author: "elm".to_string(),
        name: "core".to_string(),
        version: "1.0.5".to_string(),
        module: "List".to_string(),
        export_name: "map".to_string(),
    };

    let result = get_export_docs(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(!response.is_error);
    assert_eq!(response.content.len(), 1);

    // Parse the JSON response
    if let Some(elm_package_mcp_server::mcp::types::CallToolResultContent::Text { text }) =
        response.content.first()
    {
        let json_result: serde_json::Value = serde_json::from_str(text).unwrap();

        // Verify structure
        assert_eq!(json_result["author"], "elm");
        assert_eq!(json_result["name"], "core");
        assert_eq!(json_result["version"], "1.0.5");
        assert_eq!(json_result["module"], "List");
        assert_eq!(json_result["export_name"], "map");
        assert_eq!(json_result["export_type"], "value");

        // Verify we have a comment and type annotation
        assert!(json_result["comment"].is_string());
        assert!(!json_result["comment"].as_str().unwrap().is_empty());
        assert!(json_result["type_annotation"].is_string());
        assert!(json_result["type_annotation"]
            .as_str()
            .unwrap()
            .starts_with("map :"));
    } else {
        panic!("Expected text content");
    }
}

#[tokio::test]
async fn test_get_export_docs_for_type() {
    // Test getting comment for Maybe type
    let request = GetExportDocsRequest {
        author: "elm".to_string(),
        name: "core".to_string(),
        version: "1.0.5".to_string(),
        module: "Maybe".to_string(),
        export_name: "Maybe".to_string(),
    };

    let result = get_export_docs(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(!response.is_error);

    // Parse the JSON response
    if let Some(elm_package_mcp_server::mcp::types::CallToolResultContent::Text { text }) =
        response.content.first()
    {
        let json_result: serde_json::Value = serde_json::from_str(text).unwrap();

        assert_eq!(json_result["export_type"], "union");
        assert!(json_result["type_annotation"]
            .as_str()
            .unwrap()
            .starts_with("type Maybe"));
    } else {
        panic!("Expected text content");
    }
}

#[tokio::test]
async fn test_get_export_docs_not_found() {
    // Test with non-existent export
    let request = GetExportDocsRequest {
        author: "elm".to_string(),
        name: "core".to_string(),
        version: "1.0.5".to_string(),
        module: "List".to_string(),
        export_name: "nonExistentFunction".to_string(),
    };

    let result = get_export_docs(request).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_exports_all_modules() {
    // Test getting all exports without module filter
    let request = GetExportsRequest {
        author: "elm".to_string(),
        name: "core".to_string(),
        version: "1.0.5".to_string(),
        module: None,
    };

    let result = get_exports(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(!response.is_error);

    // Parse the JSON response
    if let Some(elm_package_mcp_server::mcp::types::CallToolResultContent::Text { text }) =
        response.content.first()
    {
        let json_result: serde_json::Value = serde_json::from_str(text).unwrap();

        // Verify we have multiple modules
        assert!(json_result["modules"].is_array());
        let modules = json_result["modules"].as_array().unwrap();
        assert!(modules.len() > 1);

        // Each module should have the expected structure
        for module in modules {
            assert!(module["name"].is_string());
            assert!(module["unions"].is_array());
            assert!(module["aliases"].is_array());
            assert!(module["values"].is_array());
            assert!(module["binops"].is_array());
        }
    } else {
        panic!("Expected text content");
    }
}
