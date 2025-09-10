# VectorStoreFileObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The identifier, which can be referenced in API endpoints. | 
**object** | **String** | The object type, which is always `vector_store.file`. | 
**usage_bytes** | **i32** | The total vector store usage in bytes. Note that this may be different from the original file size. | 
**created_at** | **i32** | The Unix timestamp (in seconds) for when the vector store file was created. | 
**vector_store_id** | **String** | The ID of the [vector store](https://platform.openai.com/docs/api-reference/vector-stores/object) that the [File](https://platform.openai.com/docs/api-reference/files) is attached to. | 
**status** | **String** | The status of the vector store file, which can be either `in_progress`, `completed`, `cancelled`, or `failed`. The status `completed` indicates that the vector store file is ready for use. | 
**last_error** | [**models::VectorStoreFileObjectLastError**](VectorStoreFileObject_last_error.md) |  | 
**chunking_strategy** | Option<[**models::ChunkingStrategyResponse**](ChunkingStrategyResponse.md)> |  | [optional]
**attributes** | Option<[**std::collections::HashMap<String, models::VectorStoreFileAttributesValue>**](VectorStoreFileAttributes_value.md)> | Set of 16 key-value pairs that can be attached to an object. This can be  useful for storing additional information about the object in a structured  format, and querying for objects via API or the dashboard. Keys are strings  with a maximum length of 64 characters. Values are strings with a maximum  length of 512 characters, booleans, or numbers.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


