use serde::{Deserialize, Serialize};

use super::ReferenceObject;

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ExampleObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dataValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_value: Option<String>,
    #[serde(rename = "serializedValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialized_value: Option<String>,
    #[serde(rename = "externalValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Example {
    Example(ExampleObject),
    Reference(ReferenceObject),
}

pub type Examples = std::collections::HashMap<String, Example>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_example_serialize_deserialize_json() {
        let example = Example::Example(ExampleObject {
            summary: Some("Example summary".to_string()),
            description: Some("Example description".to_string()),
            data_value: Some("Example data value".to_string()),
            serialized_value: Some("Example serialized value".to_string()),
            external_value: Some("Example external value".to_string()),
            value: Some("Example value".to_string()),
        });

        let serialized = serde_json::to_string(&example).unwrap();
        let deserialized: Example = serde_json::from_str(&serialized).unwrap();
        assert_eq!(example, deserialized);
    }
}
