# BetaFileSearchToolCall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> | The agent that produced this item. | [optional]
**id** | **String** | The unique ID of the file search tool call.  | 
**r#type** | **Type** | The type of the file search tool call. Always `file_search_call`.  (enum: file_search_call) | 
**status** | **Status** | The status of the file search tool call. One of `in_progress`, `searching`, `incomplete` or `failed`,  (enum: in_progress, searching, completed, incomplete, failed) | 
**queries** | **Vec<String>** | The queries used to search for files.  | 
**results** | Option<[**Vec<models::BetaFileSearchToolCallResultsInner>**](BetaFileSearchToolCallResultsInner.md)> | The results of the file search tool call.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


