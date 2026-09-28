use super::{
    Callbacks, ExternalDocumentationObject, ParameterObject, RequestBody, Responses,
    SecurityRequirementObject, ServerObject,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OperationObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,

    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterObject>>,

    #[serde(rename = "requestBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBody>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Responses>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<Callbacks>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirementObject>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
}

pub type Operations = std::collections::HashMap<String, OperationObject>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::response::{Response, ResponseObject};
    use serde_json;

    #[test]
    fn test_operation_serialize_deserialize_json() {
        let operation = OperationObject {
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            summary: Some("A summary".to_string()),
            description: Some("A description".to_string()),
            operation_id: Some("operationId".to_string()),
            deprecated: Some(false),
            responses: Some({
                let mut responses = Responses::new();
                responses.insert(
                    "200".to_string(),
                    Response::Response(ResponseObject {
                        description: Some("OK".to_string()),
                        ..Default::default()
                    }),
                );
                responses
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&operation).unwrap();
        let deserialized: OperationObject = serde_json::from_str(&json).unwrap();
        assert_eq!(operation, deserialized);
    }
}
