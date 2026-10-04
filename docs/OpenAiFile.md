# OpenAiFile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The file identifier, which can be referenced in the API endpoints. | 
**bytes** | **i32** | The size of the file, in bytes. In a completed file upload response, this can be null when the file size is not yet available.  | 
**created_at** | **i32** | The Unix timestamp (in seconds) for when the file was created. | 
**expires_at** | Option<**i32**> | The Unix timestamp (in seconds) for when the file will expire. In a completed file upload response, this can be null when no expiry is set.  | [optional]
**filename** | **String** | The name of the file. | 
**object** | **String** | The object type, which is always `file`. | 
**purpose** | **String** | The intended purpose of the file. Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`, `fine-tune`, `fine-tune-results`, `vision`, and `user_data`. | 
**status** | **String** | Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`. | 
**status_details** | Option<**String**> | Deprecated. For details on why a fine-tuning training file failed validation, see the `error` field on `fine_tuning.job`. Completed file upload responses can return null when these details are unset. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


