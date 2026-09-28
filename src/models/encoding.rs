use super::Headers;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EncodingObject {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Headers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encodings>,
    #[serde(rename = "prefixEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_encoding: Option<Box<EncodingObject>>,
    #[serde(rename = "itemEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_encoding: Option<Box<EncodingObject>>,
}

pub type Encodings = std::collections::HashMap<String, EncodingObject>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_serialize_deserialize_json() {
        let encoding = EncodingObject {
            content_type: Some("application/json".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&encoding).unwrap();
        let deserialized: EncodingObject = serde_json::from_str(&json).unwrap();
        assert_eq!(encoding, deserialized);
    }
}
