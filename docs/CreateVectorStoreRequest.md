# CreateVectorStoreRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_ids** | Option<**Vec<String>**> | A list of [File](/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files. | [optional]
**name** | Option<**String**> | The name of the vector store. | [optional]
**description** | Option<**String**> | A description for the vector store. Can be used to describe the vector store's purpose. | [optional]
**expires_after** | Option<[**models::VectorStoreExpirationAfter**](VectorStoreExpirationAfter.md)> |  | [optional]
**chunking_strategy** | Option<[**models::CreateVectorStoreRequestChunkingStrategy**](CreateVectorStoreRequest_chunking_strategy.md)> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.  Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


