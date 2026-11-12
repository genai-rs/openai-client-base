# CreateAutomationParamsPeriodic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `periodic`.  Defaults to `\"periodic\"`. (enum: periodic) | 
**name** | **String** | A human-readable name for the automation. | 
**target** | [**models::AutomationTargetParam**](AutomationTargetParam.md) |  | 
**metadata** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**expression** | **String** | A standard five-field cron expression defining scheduled occurrences. | 
**timezone** | **String** | The IANA timezone used to interpret the cron expression. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


