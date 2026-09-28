use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ReferenceObject {
    #[serde(rename = "$ref")]
    pub ref_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_reference_object_serialization_json() {
        let reference = ReferenceObject {
            ref_: "#/components/schemas/Example".to_string(),
            summary: Some("An example summary".to_string()),
            description: Some("An example description".to_string()),
        };
        let serialized = serde_json::to_string(&reference).unwrap();
        let deserialized: ReferenceObject = serde_json::from_str(&serialized).unwrap();
        assert_eq!(reference, deserialized);
    }
}
