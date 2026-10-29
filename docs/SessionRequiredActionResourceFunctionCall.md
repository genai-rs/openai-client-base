# SessionRequiredActionResourceFunctionCall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `function_call`. (enum: function_call) | 
**turn_id** | **String** | The ID of the turn that requested the function call. | 
**call_id** | **String** | The ID to include when submitting the function result. | 
**name** | **String** | The function name. | 
**arguments** | Option<**serde_json::Value**> | The arguments supplied by the model. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


