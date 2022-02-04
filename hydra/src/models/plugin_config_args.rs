/*
 * ORY Hydra
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here.
 *
 * The version of the OpenAPI document: latest
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PluginConfigArgs : PluginConfigArgs plugin config args



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PluginConfigArgs {
    /// description
    #[serde(rename = "Description")]
    pub description: String,
    /// name
    #[serde(rename = "Name")]
    pub name: String,
    /// settable
    #[serde(rename = "Settable")]
    pub settable: Vec<String>,
    /// value
    #[serde(rename = "Value")]
    pub value: Vec<String>,
}

impl PluginConfigArgs {
    /// PluginConfigArgs plugin config args
    pub fn new(description: String, name: String, settable: Vec<String>, value: Vec<String>) -> PluginConfigArgs {
        PluginConfigArgs {
            description,
            name,
            settable,
            value,
        }
    }
}


