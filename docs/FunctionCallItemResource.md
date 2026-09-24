# FunctionCallItemResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The item type. Always `function_call`. | 
**id** | **String** | The ID of the function call item. | 
**turn_id** | **String** | The ID of the turn that contains this item. | 
**call_id** | **String** | The ID used to submit the function result. | 
**name** | **String** | The name of the function to call. | 
**arguments** | Option<[**serde_json::Value**](.md)> |  | 
**status** | [**models::FunctionCallStatusResource**](FunctionCallStatusResource.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


