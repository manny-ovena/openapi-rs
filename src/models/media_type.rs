use super::ReferenceObject;

use serde::{Deserialize, Serialize};

use super::{EncodingObject, Encodings, Examples, Schema, SchemaObject};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MediaTypeObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,

    #[serde(rename = "itemSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_schema: Option<SchemaObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Examples>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encodings>,

    #[serde(rename = "prefixEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_encoding: Option<Vec<EncodingObject>>,

    #[serde(rename = "itemEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_encoding: Option<Vec<EncodingObject>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MediaType {
    MediaType(MediaTypeObject),
    Reference(ReferenceObject),
    Unknown(serde_json::Value),
}

pub type MediaTypes = std::collections::HashMap<String, MediaType>;
