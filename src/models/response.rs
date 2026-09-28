use super::{Headers, Links, MediaTypes, ReferenceObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResponseObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Headers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<MediaTypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Response {
    Response(ResponseObject),
    Reference(ReferenceObject),
}

pub type Responses = std::collections::HashMap<String, Response>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_response_serialize_deserialize_json() {
        let response = Response::Response(ResponseObject {
            summary: Some("A summary".to_string()),
            description: Some("A description".to_string()),
            ..Default::default()
        });
        let json = serde_json::to_string(&response).unwrap();
        let deserialized: Response = serde_json::from_str(&json).unwrap();
        assert_eq!(response, deserialized);
    }
}
