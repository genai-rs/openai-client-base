# VectorStoreSearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | [**models::VectorStoreSearchRequestQuery**](VectorStoreSearchRequestQuery.md) |  | 
**rewrite_query** | Option<**bool**> | Whether to rewrite the natural language query for vector search. | [optional]
**max_num_results** | Option<**i32**> | The maximum number of results to return. This number should be between 1 and 50 inclusive. | [optional]
**filters** | Option<[**models::VectorStoreSearchRequestFilters**](VectorStoreSearchRequest_filters.md)> |  | [optional]
**ranking_options** | Option<[**models::VectorStoreSearchRequestRankingOptions**](VectorStoreSearchRequest_ranking_options.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


