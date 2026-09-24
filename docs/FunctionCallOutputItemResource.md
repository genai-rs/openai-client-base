# FunctionCallOutputItemResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the function call output item. | 
**turn_id** | **String** | The ID of the turn that contains this item. | 
**r#type** | **String** | The item type. Always `function_call_output`. | 
**call_id** | **String** | The ID of the function call that produced this output. | 
**status** | [**models::FunctionCallStatusResource**](FunctionCallStatusResource.md) |  | 
**output** | Option<[**models::FunctionCallOutputResource**](FunctionCallOutputResource.md)> |  | 
**error** | Option<**String**> | The error message, if the call failed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


