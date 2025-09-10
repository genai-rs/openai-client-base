# FileSearchToolCall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique ID of the file search tool call.  | 
**r#type** | **String** | The type of the file search tool call. Always `file_search_call`.  | 
**status** | **String** | The status of the file search tool call. One of `in_progress`,  `searching`, `incomplete` or `failed`,  | 
**queries** | **Vec<String>** | The queries used to search for files.  | 
**results** | Option<[**Vec<models::FileSearchToolCallResultsInner>**](FileSearchToolCall_results_inner.md)> | The results of the file search tool call.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


