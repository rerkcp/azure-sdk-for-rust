#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "a compliance result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Compliance result data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ComplianceResultProperties>,
}
impl ComplianceResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of compliance results response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComplianceResultList {
    #[doc = "List of compliance results"]
    pub value: Vec<ComplianceResult>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ComplianceResultList {
    pub fn new(value: Vec<ComplianceResult>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Compliance result data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceResultProperties {
    #[doc = "The status of the resource regarding a single assessment"]
    #[serde(rename = "resourceStatus", default, skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<compliance_result_properties::ResourceStatus>,
}
impl ComplianceResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compliance_result_properties {
    use super::*;
    #[doc = "The status of the resource regarding a single assessment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceStatus {
        Healthy,
        NotApplicable,
        OffByPolicy,
        NotHealthy,
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
