#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "The IoT Central application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct App {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of an IoT Central application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppProperties>,
    #[doc = "Information about the SKU of the IoT Central application."]
    pub sku: AppSkuInfo,
    #[doc = "Managed service identity (either system assigned, or none)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<SystemAssignedServiceIdentity>,
}
impl App {
    pub fn new(tracked_resource: TrackedResource, sku: AppSkuInfo) -> Self {
        Self {
            tracked_resource,
            properties: None,
            sku,
            identity: None,
        }
    }
}
#[doc = "The properties indicating whether a given IoT Central application name or subdomain is available."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppAvailabilityInfo {
    #[doc = "The value which indicates whether the provided name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason for unavailability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The detailed reason message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl AppAvailabilityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of IoT Central Applications with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppListResult {
    #[doc = "The link used to get the next page of IoT Central Applications."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of IoT Central Applications."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<App>,
}
impl AppListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The description of the IoT Central application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppPatch {
    #[doc = "Instance tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Information about the SKU of the IoT Central application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AppSkuInfo>,
    #[doc = "The properties of an IoT Central application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppProperties>,
    #[doc = "Managed service identity (either system assigned, or none)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<SystemAssignedServiceIdentity>,
}
impl AppPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an IoT Central application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppProperties {
    #[doc = "The provisioning state of the application."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The ID of the application."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "The display name of the application."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The subdomain of the application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
    #[doc = "The ID of the application template, which is a blueprint that defines the characteristics and behaviors of an application. Optional; if not specified, defaults to a blank blueprint and allows the application to be defined from scratch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[doc = "The current state of the application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<AppState>,
    #[doc = "Whether requests from the public network are allowed."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[doc = "Network Rule Set Properties of this IoT Central application."]
    #[serde(rename = "networkRuleSets", default, skip_serializing_if = "Option::is_none")]
    pub network_rule_sets: Option<NetworkRuleSets>,
    #[doc = "Private endpoint connections created on this IoT Central application."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl AppProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the SKU of the IoT Central application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSkuInfo {
    #[doc = "The name of the SKU."]
    pub name: app_sku_info::Name,
}
impl AppSkuInfo {
    pub fn new(name: app_sku_info::Name) -> Self {
        Self { name }
    }
}
pub mod app_sku_info {
    use super::*;
    #[doc = "The name of the SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "ST0")]
        St0,
        #[serde(rename = "ST1")]
        St1,
        #[serde(rename = "ST2")]
        St2,
    }
}
#[doc = "The current state of the application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppState {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "suspended")]
    Suspended,
}
#[doc = "IoT Central Application Template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppTemplate {
    #[doc = "The ID of the template."]
    #[serde(rename = "manifestId", default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[doc = "The version of the template."]
    #[serde(rename = "manifestVersion", default, skip_serializing_if = "Option::is_none")]
    pub manifest_version: Option<String>,
    #[doc = "The name of the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The title of the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The order of the template in the templates list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<f64>,
    #[doc = "The description of the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The industry of the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[doc = "A list of locations that support the template."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<AppTemplateLocations>,
}
impl AppTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IoT Central Application Template Locations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppTemplateLocations {
    #[doc = "The ID of the location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name of the location."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl AppTemplateLocations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of IoT Central Application Templates with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppTemplatesResult {
    #[doc = "The link used to get the next page of IoT Central application templates."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of IoT Central Application Templates."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AppTemplate>,
}
impl AppTemplatesResult {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
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
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Whether to allow or deny network traffic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NetworkAction {
    Allow,
    Deny,
}
#[doc = "An object for an IP range that will be allowed access."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSetIpRule {
    #[doc = "The readable name of the IP rule."]
    #[serde(rename = "filterName", default, skip_serializing_if = "Option::is_none")]
    pub filter_name: Option<String>,
    #[doc = "The CIDR block defining the IP range."]
    #[serde(rename = "ipMask", default, skip_serializing_if = "Option::is_none")]
    pub ip_mask: Option<String>,
}
impl NetworkRuleSetIpRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network Rule Set Properties of this IoT Central application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSets {
    #[doc = "Whether these rules apply for device connectivity to IoT Hub and Device Provisioning service associated with this application."]
    #[serde(rename = "applyToDevices", default, skip_serializing_if = "Option::is_none")]
    pub apply_to_devices: Option<bool>,
    #[doc = "Whether these rules apply for connectivity via IoT Central web portal and APIs."]
    #[serde(rename = "applyToIoTCentral", default, skip_serializing_if = "Option::is_none")]
    pub apply_to_io_t_central: Option<bool>,
    #[doc = "Whether to allow or deny network traffic."]
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<NetworkAction>,
    #[doc = "List of IP rules."]
    #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<NetworkRuleSetIpRule>,
}
impl NetworkRuleSets {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IoT Central REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{read | write | action | delete}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "The intended executor of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Additional descriptions for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider: Microsoft IoT Central"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource Type: IoT Central"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Friendly description for the operation,"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationInputs {
    #[doc = "The name of the IoT Central application instance to check."]
    pub name: String,
    #[doc = "The type of the IoT Central resource to query."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl OperationInputs {
    pub fn new(name: String) -> Self {
        Self { name, type_: None }
    }
}
#[doc = "A list of IoT Central operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The link used to get the next page of IoT Central description objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of operations supported by the Microsoft.IoTCentral resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for private endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of private endpoint connections associated with the specified resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private endpoint connections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The group ids for the private endpoint resource."]
    #[serde(rename = "groupIds", default, skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<String>,
    #[doc = "The private endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            group_ids: Vec::new(),
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
}
#[doc = "A private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource private link DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisioning state of the application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Creating,
    Deleting,
    Updating,
    Succeeded,
    Failed,
    Canceled,
}
#[doc = "Whether requests from the public network are allowed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PublicNetworkAccess {
    Enabled,
    Disabled,
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed service identity (either system assigned, or none)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemAssignedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (either system assigned, or none)."]
    #[serde(rename = "type")]
    pub type_: SystemAssignedServiceIdentityType,
}
impl SystemAssignedServiceIdentity {
    pub fn new(type_: SystemAssignedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
        }
    }
}
#[doc = "Type of managed service identity (either system assigned, or none)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SystemAssignedServiceIdentityType {
    None,
    SystemAssigned,
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
