# FileSearchTool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the file search tool. Always `file_search`. | 
**vector_store_ids** | **Vec<String>** | The IDs of the vector stores to search. | 
**max_num_results** | Option<**i32**> | The maximum number of results to return. This number should be between 1 and 50 inclusive. | [optional]
**ranking_options** | Option<[**models::RankingOptions**](RankingOptions.md)> |  | [optional]
**filters** | Option<[**models::Filters**](Filters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


