# CustomToolCallOutputResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the custom tool call output. Always `custom_tool_call_output`.  | 
**id** | **String** | The unique ID of the custom tool call output item.  | 
**call_id** | **String** | The call ID, used to map this custom tool call output to a custom tool call.  | 
**output** | [**models::CustomToolCallOutputOutput**](CustomToolCallOutputOutput.md) |  | 
**status** | [**models::FunctionCallOutputStatusEnum**](FunctionCallOutputStatusEnum.md) |  | 
**created_by** | Option<**String**> | The identifier of the actor that created the item.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


