use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct XmlObject {
    #[serde(rename = "nodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapped: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_xml_serialize_deserialize_json() {
        let xml = XmlObject {
            node_type: Some("element".to_string()),
            name: Some("test".to_string()),
            namespace: Some("http://example.com".to_string()),
            prefix: Some("ex".to_string()),
            attribute: Some(true),
            wrapped: Some(false),
        };
        let json = serde_json::to_string(&xml).unwrap();
        let deserialized: XmlObject = serde_json::from_str(&json).unwrap();
        assert_eq!(xml, deserialized);
    }
}
