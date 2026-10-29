# LiveFunctionToolInputParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The tool type. Always `function`. (enum: function) | 
**name** | **String** | The name the delegated Responses model uses when calling this function. | 
**description** | Option<**String**> | What the function does and when the delegated Responses model should call it. | [optional]
**parameters** | Option<**std::collections::HashMap<String, serde_json::Value>**> | A JSON Schema object describing the arguments accepted by the function. | [optional]
**strict** | Option<**bool**> | Whether the delegated Responses model must follow the function’s parameter schema exactly. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


