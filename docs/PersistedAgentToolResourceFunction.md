# PersistedAgentToolResourceFunction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `function`. (enum: function) | 
**name** | **String** | The name of the function. | 
**description** | **String** | A description of what the function does. | 
**parameters** | **std::collections::HashMap<String, serde_json::Value>** | A JSON Schema object describing the function's arguments. | 
**defer_loading** | **bool** | Whether the function is deferred and discovered through tool search. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


