use super::{DiscriminatorObject, ReferenceObject};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SchemaObject {
    AnyOf {
        #[serde(rename = "anyOf")]
        any_of: Vec<SchemaObject>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    AllOf {
        #[serde(rename = "allOf")]
        all_of: Vec<SchemaObject>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    OneOf {
        #[serde(rename = "oneOf")]
        one_of: Vec<SchemaObject>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    SchemaType(SchemaType),
    Reference(ReferenceObject),
    Discriminator(DiscriminatorObject),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SchemaType {
    Array {
        items: Box<SchemaObject>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        min_items: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_items: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        unique_items: Option<bool>,
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    Boolean {
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    String {
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pattern: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        min_length: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_length: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        minimum: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        maximum: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    Integer {
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        minimum: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        maximum: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    Object {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        properties: Option<IndexMap<String, SchemaObject>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<Vec<String>>,
        #[serde(rename = "additionalProperties")]
        #[serde(skip_serializing_if = "Option::is_none")]
        additional_properties: Option<AdditionalProperties>,
        #[serde(flatten)]
        extensions: IndexMap<String, serde_json::Value>,
    },
    Null {
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Boolean(bool),
    Schema(Box<SchemaObject>),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Schema {
    Schema(SchemaObject),
    Reference(ReferenceObject),
}

pub type Schemas = std::collections::HashMap<String, Schema>;
