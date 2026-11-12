# CreateAutomationParamsOnce

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `once`.  Defaults to `\"once\"`. (enum: once) | 
**name** | **String** | A human-readable name for the automation. | 
**target** | [**models::AutomationTargetParam**](AutomationTargetParam.md) |  | 
**metadata** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**run_at** | **i64** | A future Unix timestamp in seconds. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


