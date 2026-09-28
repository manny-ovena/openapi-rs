use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LicenseObject {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_license_serialize_deserialize_json() {
        let license = LicenseObject {
            name: "Example License".to_string(),
            url: Some("http://example.com/license".to_string()),
        };

        let serialized = serde_json::to_string(&license).unwrap();
        let deserialized: LicenseObject = serde_json::from_str(&serialized).unwrap();
        assert_eq!(license, deserialized);
    }
}
