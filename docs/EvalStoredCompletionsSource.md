# EvalStoredCompletionsSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of source. Always `stored_completions`. | 
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.  Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | [optional]
**model** | Option<**String**> | ID of the model to use | [optional]
**created_after** | Option<**i32**> | An optional Unix timestamp to filter items created after this time. | [optional]
**created_before** | Option<**i32**> | An optional Unix timestamp to filter items created before this time. | [optional]
**limit** | Option<**i32**> | An optional maximum number of items to return. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


