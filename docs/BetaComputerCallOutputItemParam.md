# BetaComputerCallOutputItemParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTagParam**](Beta_AgentTagParam.md)> |  | [optional]
**id** | Option<**String**> | The ID of the computer tool call output. | [optional]
**call_id** | **String** | The ID of the computer tool call that produced the output. | 
**r#type** | **String** | The type of the computer tool call output. Always `computer_call_output`. | 
**output** | [**models::BetaComputerScreenshotImage**](BetaComputerScreenshotImage.md) |  | 
**acknowledged_safety_checks** | Option<[**Vec<models::BetaComputerCallSafetyCheckParam>**](BetaComputerCallSafetyCheckParam.md)> | The safety checks reported by the API that have been acknowledged by the developer. | [optional]
**status** | Option<[**models::BetaFunctionCallItemStatus**](BetaFunctionCallItemStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


