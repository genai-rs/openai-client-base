# ComputerToolCall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the computer call. Always `computer_call`. | 
**id** | **String** | The unique ID of the computer call. | 
**call_id** | **String** | An identifier used when responding to the tool call with output.  | 
**action** | [**serde_json::Value**](.md) |  | 
**pending_safety_checks** | [**Vec<models::ComputerCallSafetyCheckParam>**](ComputerCallSafetyCheckParam.md) | The pending safety checks for the computer call.  | 
**status** | **String** | The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


