use super::{ContactObject, LicenseObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InfoObject {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "termsOfService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_info_serialize_deserialize_json() {
        let info = InfoObject {
            title: "Example title".to_string(),
            summary: Some("Example summary".to_string()),
            description: Some("Example description".to_string()),
            terms_of_service: Some("http://example.com/terms/".to_string()),
            version: Some("1.0.0".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&info).unwrap();
        let deserialized: InfoObject = serde_json::from_str(&serialized).unwrap();
        assert_eq!(info, deserialized);
    }
}
