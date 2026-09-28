use super::{Examples, ReferenceObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ParameterObject {
    pub name: String,
    #[serde(rename = "in")]
    pub in_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Examples>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Parameter {
    Parameter(ParameterObject),
    Reference(ReferenceObject),
}

pub type Parameters = std::collections::HashMap<String, Parameter>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_parameter_serialize_deserialize_json() {
        let parameter = ParameterObject {
            name: "param".to_string(),
            in_: "query".to_string(),
            description: Some("A description".to_string()),
            required: Some(true),
            deprecated: Some(false),
            allow_empty_value: Some(false),
            example: Some("example".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&parameter).unwrap();
        let deserialized: ParameterObject = serde_json::from_str(&json).unwrap();
        assert_eq!(parameter, deserialized);
    }
}
