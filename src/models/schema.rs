use super::{DiscriminatorObject, ReferenceObject};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SchemaObject {
    AnyOf(AnyOfType),
    AllOf(AllOfType),
    OneOf(OneOfType),
    SchemaType(SchemaType),
    Reference(ReferenceObject),
    Discriminator(DiscriminatorObject),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AnyOfType {
    #[serde(rename = "anyOf")]
    pub any_of: Vec<SchemaObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OneOfType {
    #[serde(rename = "oneOf")]
    pub one_of: Vec<SchemaObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AllOfType {
    #[serde(rename = "allOf")]
    pub all_of: Vec<SchemaObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SchemaType {
    Array(ArrayType),
    Boolean(BooleanType),
    String(StringType),
    Number(NumberType),
    Integer(IntegerType),
    Object(ObjectType),
    Null(NullType),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ArrayType {
    pub items: Box<SchemaObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BooleanType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct StringType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct NullType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct NumberType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct IntegerType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ObjectType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IndexMap<String, SchemaObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(rename = "additionalProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
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
