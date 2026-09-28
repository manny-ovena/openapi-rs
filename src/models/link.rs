use serde::{Deserialize, Serialize};

use super::{ReferenceObject, ServerObject};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LinkObject {
    pub operation_ref: Option<String>,
    pub operation_id: Option<String>,
    pub parameters: Option<std::collections::HashMap<String, String>>,
    pub request_body: Option<String>,
    pub description: Option<String>,
    pub server: Option<ServerObject>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Link {
    Link(LinkObject),
    Reference(ReferenceObject),
}

pub type Links = std::collections::HashMap<String, Link>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_link_serialize_deserialize_json() {
        let link = LinkObject {
            operation_ref: Some("http://example.com/operation".to_string()),
            operation_id: Some("exampleOperationId".to_string()),
            parameters: Some(std::collections::HashMap::new()),
            request_body: Some("exampleRequestBody".to_string()),
            description: Some("Example description".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&link).unwrap();
        let deserialized: LinkObject = serde_json::from_str(&serialized).unwrap();
        assert_eq!(link, deserialized);
    }
}
