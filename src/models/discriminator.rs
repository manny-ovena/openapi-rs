use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DiscriminatorObject {
    #[serde(rename = "propertyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<std::collections::HashMap<String, String>>,

    #[serde(rename = "defaultMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mapping: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_discriminator_serialize_deserialize_json() {
        let discriminator = DiscriminatorObject {
            property_name: Some("type".to_string()),
            mapping: Some(std::collections::HashMap::new()),
            default_mapping: Some("default".to_string()),
        };

        let json = serde_json::to_string(&discriminator).unwrap();
        let deserialized: DiscriminatorObject = serde_json::from_str(&json).unwrap();
        assert_eq!(discriminator, deserialized);
    }
}
