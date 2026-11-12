# AutomationResourcePeriodic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `periodic`.  Defaults to `\"periodic\"`. (enum: periodic) | 
**id** | **String** | The automation ID. | 
**object** | **Object** | Always `agent.automation`.  Defaults to `\"agent.automation\"`. (enum: agent.automation) | 
**created_at** | **i64** | The creation time as a Unix timestamp in seconds. | 
**name** | **String** | The automation's display name. | 
**target** | [**models::AutomationTargetResource**](AutomationTargetResource.md) |  | 
**metadata** | **std::collections::HashMap<String, String>** | Custom string key-value pairs attached to the automation. | 
**created_by** | [**models::AutomationCreatedByResource**](AutomationCreatedByResource.md) |  | 
**expression** | **String** | The cron expression determining scheduled occurrences. | 
**timezone** | **String** | The timezone used to interpret the expression. | 
**status** | [**models::PeriodicAutomationStatusResource**](PeriodicAutomationStatusResource.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


