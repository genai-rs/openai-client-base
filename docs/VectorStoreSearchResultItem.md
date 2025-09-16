# VectorStoreSearchResultItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the vector store file. | 
**filename** | **String** | The name of the vector store file. | 
**score** | **f64** | The similarity score for the result. | 
**attributes** | Option<[**std::collections::HashMap<String, models::VectorStoreFileAttributesValue>**](VectorStoreFileAttributes_value.md)> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard. Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters, booleans, or numbers.  | 
**content** | [**Vec<models::VectorStoreSearchResultContentObject>**](VectorStoreSearchResultContentObject.md) | Content chunks from the file. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


