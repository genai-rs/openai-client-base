# VectorStoreFileContentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | The object type, which is always `vector_store.file_content.page` (enum: vector_store.file_content.page) | 
**data** | [**Vec<models::VectorStoreFileContentResponseDataInner>**](VectorStoreFileContentResponseDataInner.md) | Parsed content of the file. | 
**has_more** | **bool** | Indicates if there are more content pages to fetch. | 
**next_page** | Option<**String**> | The token for the next page, if any. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


