use serde::{Deserialize, Serialize};

use super::{
    Callbacks, Examples, Headers, Links, MediaTypes, Parameters, PathItems, RequestBodies,
    Responses, Schemas, SecuritySchemes,
};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ComponentsObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Schemas>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Responses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Examples>,
    #[serde(rename = "requestBodies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_bodies: Option<RequestBodies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Headers>,
    #[serde(rename = "securitySchemes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_schemes: Option<SecuritySchemes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<Callbacks>,
    #[serde(rename = "pathItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_items: Option<PathItems>,
    #[serde(rename = "mediaTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_types: Option<MediaTypes>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_components_serialize_deserialize_json() {
        let components = ComponentsObject::default();
        let json = serde_json::to_string(&components).unwrap();
        let deserialized: ComponentsObject = serde_json::from_str(&json).unwrap();
        assert_eq!(components, deserialized);
    }
}
