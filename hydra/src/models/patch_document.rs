/*
 * ORY Hydra
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here.
 *
 * The version of the OpenAPI document: latest
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchDocument : A JSONPatch document as defined by RFC 6902



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PatchDocument {
    /// A JSON-pointer
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The operation to be performed
    #[serde(rename = "op")]
    pub op: String,
    /// A JSON-pointer
    #[serde(rename = "path")]
    pub path: String,
    /// The value to be used within the operations
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl PatchDocument {
    /// A JSONPatch document as defined by RFC 6902
    pub fn new(op: String, path: String) -> PatchDocument {
        PatchDocument {
            from: None,
            op,
            path,
            value: None,
        }
    }
}

