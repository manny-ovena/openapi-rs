use super::{OAuthFlowObject, ReferenceObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SecuritySchemeObject {
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_: Option<String>,

    pub scheme: String,

    #[serde(rename = "bearerFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<OAuthFlowObject>,

    #[serde(rename = "openIdConnectUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_url: Option<String>,

    #[serde(rename = "oauth2MetadataUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_metadata_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SecurityScheme {
    SecurityScheme(SecuritySchemeObject),
    Reference(ReferenceObject),
}

pub type SecuritySchemes = std::collections::HashMap<String, SecurityScheme>;
