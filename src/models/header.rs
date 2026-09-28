use super::{Examples, ReferenceObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HeaderObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Examples>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Header {
    Header(HeaderObject),
    Reference(ReferenceObject),
}

pub type Headers = std::collections::HashMap<String, Header>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_header_serialize_deserialize_json() {
        let header = Header::Header(HeaderObject {
            description: Some("Example description".to_string()),
            required: Some(true),
            deprecated: Some(false),
            example: Some("Example".to_string()),
            ..Default::default()
        });

        let serialized = serde_json::to_string(&header).unwrap();
        let deserialized: Header = serde_json::from_str(&serialized).unwrap();
        assert_eq!(header, deserialized);
    }
}
