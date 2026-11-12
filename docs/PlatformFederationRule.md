# PlatformFederationRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | Defaults to `\"organization.workload_identity_federation_rule\"`. (enum: organization.workload_identity_federation_rule) | 
**id** | **String** |  | 
**workload_identity_provider_id** | **String** |  | 
**project_id** | **String** |  | 
**service_account_id** | **String** |  | 
**name** | Option<**String**> |  | 
**description** | Option<**String**> |  | 
**attribute_matchers** | [**std::collections::HashMap<String, models::PlatformFederationRuleAttributeMatchersValue>**](PlatformFederationRuleAttributeMatchersValue.md) |  | 
**permissions** | [**models::PlatformFederationRulePermissions**](PlatformFederationRulePermissions.md) |  | 
**enabled** | **bool** |  | 
**created_at** | **i64** |  | 
**updated_at** | **i64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


