use serde::{Deserialize, Serialize};

use super::{OperationObject, Operations, Parameter, ServerObject};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PathItemObject {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<OperationObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<OperationObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<OperationObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<OperationObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OperationObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<OperationObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<OperationObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<OperationObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<OperationObject>,

    #[serde(rename = "additionalOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_operations: Option<Operations>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

pub type PathItems = std::collections::HashMap<String, PathItemObject>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_path_item_serialize_deserialize_json() {
        let path_item = PathItemObject {
            ref_: Some("#/components/pathItems/somePathItem".to_string()),
            summary: Some("A summary".to_string()),
            description: Some("A description".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&path_item).unwrap();
        let deserialized: PathItemObject = serde_json::from_str(&json).unwrap();
        assert_eq!(path_item, deserialized);
    }
}
