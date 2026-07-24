# BetaComputerToolCallOutputResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**r#type** | **String** | The type of the computer tool call output. Always `computer_call_output`.  | 
**id** | **String** | The unique ID of the computer call tool output.  | 
**call_id** | **String** | The ID of the computer tool call that produced the output.  | 
**acknowledged_safety_checks** | Option<[**Vec<models::BetaComputerCallSafetyCheckParam>**](BetaComputerCallSafetyCheckParam.md)> | The safety checks reported by the API that have been acknowledged by the developer.  | [optional]
**output** | [**models::BetaComputerScreenshotImage**](BetaComputerScreenshotImage.md) |  | 
**status** | [**models::BetaComputerCallOutputStatus**](BetaComputerCallOutputStatus.md) |  | 
**created_by** | Option<**String**> | The identifier of the actor that created the item.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


