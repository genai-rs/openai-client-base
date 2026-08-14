# BetaFunctionTool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the function tool. Always `function`. | 
**name** | **String** | The name of the function to call. | 
**description** | Option<**String**> | A description of the function. Used by the model to determine whether or not to call the function. | [optional]
**parameters** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | A JSON schema object describing the parameters of the function. | 
**output_schema** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | A JSON schema object describing the JSON value encoded in string outputs for this function. | [optional]
**strict** | Option<**bool**> | Whether strict parameter validation is enforced for this function tool. | 
**defer_loading** | Option<**bool**> | Whether this function is deferred and loaded via tool search. | [optional]
**allowed_callers** | Option<[**Vec<models::BetaCallableToolAllowedCaller>**](BetaCallableToolAllowedCaller.md)> | The tool invocation context(s). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


