# VectorStoreSearchResultsPage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | The object type, which is always `vector_store.search_results.page` | 
**search_query** | **Vec<String>** |  | 
**data** | [**Vec<models::VectorStoreSearchResultItem>**](VectorStoreSearchResultItem.md) | The list of search result items. | 
**has_more** | **bool** | Indicates if there are more results to fetch. | 
**next_page** | **String** | The token for the next page, if any. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


