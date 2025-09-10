# FunctionTool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the function tool. Always `function`. | 
**name** | **String** | The name of the function to call. | 
**description** | Option<**String**> | A description of the function. Used by the model to determine whether or not to call the function. | [optional]
**parameters** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | A JSON schema object describing the parameters of the function. | 
**strict** | Option<**bool**> | Whether to enforce strict parameter validation. Default `true`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


