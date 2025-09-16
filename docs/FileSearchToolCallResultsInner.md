# FileSearchToolCallResultsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_id** | Option<**String**> | The unique ID of the file.  | [optional]
**text** | Option<**String**> | The text that was retrieved from the file.  | [optional]
**filename** | Option<**String**> | The name of the file.  | [optional]
**attributes** | Option<[**std::collections::HashMap<String, models::VectorStoreFileAttributesValue>**](VectorStoreFileAttributes_value.md)> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard. Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters, booleans, or numbers.  | [optional]
**score** | Option<**f32**> | The relevance score of the file - a value between 0 and 1.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


