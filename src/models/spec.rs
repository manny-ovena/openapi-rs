use super::{
    ComponentsObject, ExternalDocumentationObject, InfoObject, PathItems,
    SecurityRequirementObject, ServerObject, TagObject, Webhooks,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Spec {
    pub openapi: String,
    #[serde(rename = "$self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    pub info: InfoObject,
    #[serde(rename = "jsonSchemaDialect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema_dialect: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<PathItems>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Webhooks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<ComponentsObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirementObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagObject>>,
    #[serde(rename = "externalDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_spec_serialize_deserialize_json() {
        let spec = Spec {
            openapi: "3.0.0".to_string(),
            self_: Some("self".to_string()),
            info: InfoObject {
                title: "API Title".to_string(),
                version: Some("1.0.0".to_string()),
                ..Default::default()
            },
            json_schema_dialect: Some("http://json-schema.org/draft-07/schema#".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&spec).unwrap();
        let deserialized: Spec = serde_json::from_str(&json).unwrap();
        assert_eq!(spec, deserialized);
    }
}
