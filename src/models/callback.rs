use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CallbackObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    url: String,
}

pub type Callbacks = std::collections::HashMap<String, CallbackObject>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_callback_serialize_deserialize_json() {
        let callback = CallbackObject {
            description: Some("Example description".to_string()),
            url: "http://example.com/callback".to_string(),
        };

        let serialized = serde_json::to_string(&callback).unwrap();
        let deserialized: CallbackObject = serde_json::from_str(&serialized).unwrap();
        assert_eq!(callback, deserialized);
    }
}
