# BetaComputerToolCallOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**r#type** | **String** | The type of the computer tool call output. Always `computer_call_output`.  | 
**id** | Option<**String**> | The ID of the computer tool call output.  | [optional]
**call_id** | **String** | The ID of the computer tool call that produced the output.  | 
**acknowledged_safety_checks** | Option<[**Vec<models::BetaComputerCallSafetyCheckParam>**](BetaComputerCallSafetyCheckParam.md)> | The safety checks reported by the API that have been acknowledged by the developer.  | [optional]
**output** | [**models::BetaComputerScreenshotImage**](BetaComputerScreenshotImage.md) |  | 
**status** | Option<**String**> | The status of the message input. One of `in_progress`, `completed`, or `incomplete`. Populated when input items are returned via API.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


