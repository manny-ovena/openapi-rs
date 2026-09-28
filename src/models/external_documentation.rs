use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ExternalDocumentationObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_external_documentation_serialize_deserialize_json() {
        let external_doc = ExternalDocumentationObject {
            description: Some("Example description".to_string()),
            url: "http://example.com".to_string(),
        };

        let serialized = serde_json::to_string(&external_doc).unwrap();
        let deserialized: ExternalDocumentationObject = serde_json::from_str(&serialized).unwrap();
        assert_eq!(external_doc, deserialized);
    }
}
