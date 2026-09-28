use super::ExternalDocumentationObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TagObject {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_tagobject_serialize_deserialize_json() {
        let tag_object = TagObject {
            name: "tag_name".to_string(),
            summary: Some("A summary".to_string()),
            description: Some("A description".to_string()),
            parent: Some("parent_tag".to_string()),
            kind: Some("tag_kind".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&tag_object).unwrap();
        let deserialized: TagObject = serde_json::from_str(&json).unwrap();
        assert_eq!(tag_object, deserialized);
    }
}
