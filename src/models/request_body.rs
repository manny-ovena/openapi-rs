use serde::{Deserialize, Serialize};

use super::{MediaTypes, ReferenceObject};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RequestBodyObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub content: MediaTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RequestBody {
    RequestBody(RequestBodyObject),
    Reference(ReferenceObject),
}

pub type RequestBodies = std::collections::HashMap<String, RequestBody>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_request_body_serialize_deserialize_json() {
        let request_body = RequestBody::RequestBody(RequestBodyObject {
            description: Some("A description".to_string()),
            content: std::collections::HashMap::new(),
            required: Some(true),
        });
        let json = serde_json::to_string(&request_body).unwrap();
        let deserialized: RequestBody = serde_json::from_str(&json).unwrap();
        assert_eq!(request_body, deserialized);
    }

    #[test]
    fn test_request_body_serialize_deserialize_json_with_ref() {
        let request_body = RequestBody::Reference(ReferenceObject {
            ref_: "#/components/requestBodies/MyRequestBody".to_string(),
            description: Some("A description".to_string()),
            summary: Some("A summary".to_string()),
        });
        let json = serde_json::to_string(&request_body).unwrap();
        let deserialized: RequestBody = serde_json::from_str(&json).unwrap();
        assert_eq!(request_body, deserialized);
    }
}
