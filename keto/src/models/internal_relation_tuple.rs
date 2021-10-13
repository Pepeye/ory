/*
 * ORY Keto
 *
 * Ory Keto is a cloud native access control server providing best-practice patterns (RBAC, ABAC, ACL, AWS IAM Policies, Kubernetes Roles, ...) via REST APIs.
 *
 * The version of the OpenAPI document: Latest
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InternalRelationTuple {
    /// Namespace of the Relation Tuple
    #[serde(rename = "namespace")]
    pub namespace: String,
    /// Object of the Relation Tuple
    #[serde(rename = "object")]
    pub object: String,
    /// Relation of the Relation Tuple
    #[serde(rename = "relation")]
    pub relation: String,
    /// SubjectID of the Relation Tuple  Either SubjectSet or SubjectID are required.
    #[serde(rename = "subject_id", skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    #[serde(rename = "subject_set", skip_serializing_if = "Option::is_none")]
    pub subject_set: Option<Box<crate::models::SubjectSet>>,
}

impl InternalRelationTuple {
    pub fn new(namespace: String, object: String, relation: String) -> InternalRelationTuple {
        InternalRelationTuple {
            namespace,
            object,
            relation,
            subject_id: None,
            subject_set: None,
        }
    }
}

