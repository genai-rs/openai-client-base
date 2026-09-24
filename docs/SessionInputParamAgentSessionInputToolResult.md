# SessionInputParamAgentSessionInputToolResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `agent.session.input.tool_result`. | 
**turn_id** | **String** | The ID of the turn that requested the function call. | 
**call_id** | **String** | The ID of the function call. | 
**success** | **bool** | Whether the function call succeeded. | 
**output** | Option<[**models::FunctionCallOutputParam**](FunctionCallOutputParam.md)> |  | [optional]
**error** | Option<**String**> | The error message when the call failed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


