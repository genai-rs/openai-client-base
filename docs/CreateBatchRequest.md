# CreateBatchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**input_file_id** | **String** | The ID of an uploaded file that contains requests for the new batch.  See [upload file](https://platform.openai.com/docs/api-reference/files/create) for how to upload a file.  Your input file must be formatted as a [JSONL file](https://platform.openai.com/docs/api-reference/batch/request-input), and must be uploaded with the purpose `batch`. The file can contain up to 50,000 requests, and can be up to 200 MB in size.  | 
**endpoint** | **String** | The endpoint to be used for all requests in the batch. Currently `/v1/responses`, `/v1/chat/completions`, `/v1/embeddings`, and `/v1/completions` are supported. Note that `/v1/embeddings` batches are also restricted to a maximum of 50,000 embedding inputs across all requests in the batch. | 
**completion_window** | **String** | The time frame within which the batch should be processed. Currently only `24h` is supported. | 
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.   Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | [optional]
**output_expires_after** | Option<[**models::BatchFileExpirationAfter**](BatchFileExpirationAfter.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


