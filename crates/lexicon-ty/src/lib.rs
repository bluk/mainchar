//! Types for deserializing [Lexicon][lexicon] schema definitions.
//!
//! [lexicon]: https://atproto.com/specs/lexicon

#![allow(missing_docs)]

use std::collections::BTreeMap;

use serde::Deserialize;

/// Schema definition
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Lexicon {
    /// Lexicon language version
    pub lexicon: u32,
    /// `NSID`
    pub id: String,
    // pub revision: Option<u32>,
    /// Overview
    pub description: Option<String>,
    /// Definitions of types
    pub defs: BTreeMap<String, Def>,
}

/// Type definition
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", deny_unknown_fields)]
pub enum Def {
    /// XRPC Query (HTTP GET)
    Query(Query),
    /// XRPC Procedure (HTTP POST)
    Procedure(Procedure),
    /// Event Stream (WebSocket)
    Subscription(Subscription),
    /// Object that can be stored in a repository record
    Record(Record),

    String(StringTy),
    Token(Token),
    Object(Object),
    Array(Array),
}

/// Object that can be stored in a repository record
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Record {
    pub description: Option<String>,
    /// Record Key Type
    pub key: String,
    /// Schema definition, (only `object` supported)
    pub record: RecordSchema,
}

/// Record schema definition
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", deny_unknown_fields)]
pub enum RecordSchema {
    Object(Object),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Query {
    pub description: Option<String>,
    /// HTTP query parameters
    pub parameters: Option<Params>,
    /// HTTP response body
    pub output: Option<Output>,
    /// Set of string error codes which may be returned
    pub errors: Option<Vec<Error>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Procedure {
    pub description: Option<String>,
    /// HTTP response body
    pub output: Option<Output>,
    /// HTTP request body
    pub input: Option<Input>,
    /// Set of string error codes which may be returned
    pub errors: Option<Vec<Error>>,
}

/// HTTP response body
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Output {
    // description: Option<String>,
    /// MIME type for body contents.
    pub encoding: String,
    /// Schema definition
    pub schema: Option<OutputSchema>,
}

/// Schema definition
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", deny_unknown_fields)]
pub enum OutputSchema {
    Object(Object),
    Ref(Ref),
    // Union(Union),
}

/// HTTP request body
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Input {
    // description: Option<String>,
    /// MIME type for body contents.
    pub encoding: String,
    /// Schema definition
    pub schema: Option<InputSchema>,
}

/// Schema definition
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", deny_unknown_fields)]
pub enum InputSchema {
    Object(Object),
    Ref(Ref),
    // Union(Union),
}

/// String error codes
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Error {
    /// Short name for error type (no whitespace)
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Subscription {
    pub description: Option<String>,
    /// HTTP query parameters
    pub parameters: Option<Params>,
    /// Type of messages which can be sent
    pub message: Option<Message>,
    /// Set of string error codes which may be returned
    pub errors: Option<Vec<Error>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Message {
    // description: Option<String>,
    /// Schema definition, (only `union` supported)
    pub schema: MessageSchema,
}

/// Message schema definition
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", deny_unknown_fields)]
pub enum MessageSchema {
    Union(Union),
}

// Null field type is not used

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Boolean {
    pub description: Option<String>,
    /// Default value
    pub default: Option<bool>,
    /// Fixed value
    #[serde(rename = "const")]
    pub constant: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Integer {
    pub description: Option<String>,
    /// Min valid value
    pub minimum: Option<i64>,
    /// Max valid value
    pub maximum: Option<i64>,
    // #[serde(rename = "enum")]
    // enum_values: Option<Vec<i64>>,
    /// Default value
    pub default: Option<i64>,
    // #[serde(rename = "const")]
    // constant: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StringTy {
    pub description: Option<String>,
    /// Format restriction
    pub format: Option<StringFormat>,
    /// Max length in UTF-8 bytes
    pub max_length: Option<u64>,
    /// Min length in UTF-8 bytes
    pub min_length: Option<u64>,
    /// Max length of value in Unicode Grapheme Clusters
    pub max_graphemes: Option<u64>,
    // min_graphemes: Option<u64>,
    /// Suggested/comment values (but not only values)
    pub known_values: Option<Vec<String>>,
    /// Closed set of allowed values
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
    /// Default value
    pub default: Option<String>,
    // #[serde(rename = "const")]
    // constant: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum StringFormat {
    #[serde(rename = "at-identifier")]
    AtIdentifier,
    #[serde(rename = "at-uri")]
    AtUri,
    Cid,
    Datetime,
    Did,
    Handle,
    Nsid,
    Uri,
    Language,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Bytes {
    pub description: Option<String>,
    // min_length: Option<u64>,
    /// Max size in bytes
    pub max_length: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CidLink {
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Array {
    pub description: Option<String>,
    /// Schema for array elements
    pub items: ItemsSchema,
    /// Min count of elements
    pub min_length: Option<u64>,
    /// Max count of elements
    pub max_length: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", deny_unknown_fields)]
pub enum ItemsSchema {
    #[serde(rename = "cid-link")]
    CidLink(CidLink),
    Integer(Integer),
    Ref(Ref),
    String(StringTy),
    Union(Union),
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Object {
    pub description: Option<String>,
    /// Schema for each field by property name
    pub properties: BTreeMap<String, FieldSchema>,
    /// Array of property names which are required
    pub required: Option<Vec<String>>,
    /// Array of property names which are nullable
    pub nullable: Option<Vec<String>>,
}

impl Object {
    #[must_use]
    pub fn is_required(&self, name: &str) -> bool {
        let Some(required) = &self.required else {
            return false;
        };

        required.iter().any(|v| v == name)
    }

    #[must_use]
    pub fn is_nullable(&self, name: &str) -> bool {
        let Some(nullable) = &self.nullable else {
            return false;
        };

        nullable.iter().any(|v| v == name)
    }

    #[must_use]
    pub fn has_reference_field(&self) -> bool {
        self.properties.values().any(FieldSchema::can_be_reference)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum FieldSchema {
    Boolean(Boolean),
    Integer(Integer),
    String(StringTy),
    Bytes(Bytes),
    #[serde(rename = "cid-link")]
    CidLink(CidLink),
    Array(Array),
    Blob(Blob),
    Ref(Ref),
    Union(Union),
    Unknown,
}

impl FieldSchema {
    fn can_be_reference(&self) -> bool {
        match self {
            FieldSchema::Boolean(_)
            | FieldSchema::Integer(_)
            | FieldSchema::Unknown
            | FieldSchema::Bytes(_)
            | FieldSchema::CidLink(_)
            | FieldSchema::Array(_)
            | FieldSchema::Blob(_)
            | FieldSchema::Ref(_)
            | FieldSchema::Union(_) => false,
            FieldSchema::String(_) => true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Blob {
    pub description: Option<String>,
    /// Array of acceptable MIME types
    pub accept: Option<Vec<String>>,
    /// Max size in bytes
    pub max_size: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Params {
    // pub description: Option<String>,
    #[serde(rename = "type")]
    pub ty: String,
    /// Array of property names which are required
    pub required: Option<Vec<String>>,
    /// Schema for each field by property name
    pub properties: BTreeMap<String, ParamSchema>,
}

impl Params {
    #[must_use]
    pub fn is_required(&self, name: &str) -> bool {
        let Some(required) = &self.required else {
            return false;
        };

        required.iter().any(|v| v == name)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ParamSchema {
    Boolean(Boolean),
    Integer(Integer),
    String(StringTy),
    // Unknown,
    Array(ParamArray),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParamArray {
    pub description: Option<String>,
    /// Schema for array elements
    pub items: ParamItemsSchema,
    /// Min count of elements
    pub min_length: Option<u64>,
    /// Max count of elements
    pub max_length: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", deny_unknown_fields)]
pub enum ParamItemsSchema {
    // Boolean(Boolean),
    // Integer(Integer),
    String(StringTy),
    // Unknown
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Token {
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Ref {
    pub description: Option<String>,
    /// Reference to another schema definition
    #[serde(rename = "ref")]
    pub reference: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Union {
    pub description: Option<String>,
    /// If union is limited to only listed references
    #[serde(default)]
    pub closed: bool,
    /// Set of references to other schema definitions
    pub refs: Vec<String>,
}
