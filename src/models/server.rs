use super::ServerVariables;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerObject {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<ServerVariables>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serverobject_serialize_deserialize_json() {
        let server_object = ServerObject {
            url: "https://example.com".to_string(),
            description: Some("A description".to_string()),
            name: Some("server_name".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&server_object).unwrap();
        let deserialized: ServerObject = serde_json::from_str(&json).unwrap();
        assert_eq!(server_object, deserialized);
    }
}
