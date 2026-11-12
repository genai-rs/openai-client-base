# UpdateAutomationParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**target** | Option<[**models::AutomationTargetParam**](AutomationTargetParam.md)> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | Replaces all metadata; omit to leave unchanged. | [optional]
**expression** | Option<**String**> | Only accepted for periodic automations. | [optional]
**timezone** | Option<**String**> | Only accepted for periodic automations. | [optional]
**run_at** | Option<**i64**> | A future Unix timestamp in seconds; accepted only before a one-time scheduled invocation exists. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


