use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerVariable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    pub default: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

pub type ServerVariables = std::collections::HashMap<String, ServerVariable>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_servervariable_serialize_deserialize_json() {
        let server_variable = ServerVariable {
            enum_values: Some(vec!["value1".to_string(), "value2".to_string()]),
            default: "value1".to_string(),
            description: Some("A description".to_string()),
        };

        let json = serde_json::to_string(&server_variable).unwrap();
        let deserialized: ServerVariable = serde_json::from_str(&json).unwrap();
        assert_eq!(server_variable, deserialized);
    }
}
