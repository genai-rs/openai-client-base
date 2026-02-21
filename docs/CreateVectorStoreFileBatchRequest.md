# CreateVectorStoreFileBatchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_ids** | Option<**Vec<String>**> | A list of [File](/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files.  If `attributes` or `chunking_strategy` are provided, they will be  applied to all files in the batch. The maximum batch size is 2000 files. Mutually exclusive with `files`. | [optional]
**files** | Option<[**Vec<models::CreateVectorStoreFileRequest>**](CreateVectorStoreFileRequest.md)> | A list of objects that each include a `file_id` plus optional `attributes` or `chunking_strategy`. Use this when you need to override metadata for specific files. The global `attributes` or `chunking_strategy` will be ignored and must be specified for each file. The maximum batch size is 2000 files. Mutually exclusive with `file_ids`. | [optional]
**chunking_strategy** | Option<[**models::ChunkingStrategyRequestParam**](ChunkingStrategyRequestParam.md)> |  | [optional]
**attributes** | Option<[**std::collections::HashMap<String, models::VectorStoreFileAttributesValue>**](VectorStoreFileAttributes_value.md)> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard. Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters, booleans, or numbers.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


