# Batch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**object** | **String** | The object type, which is always `batch`. | 
**endpoint** | **String** | The OpenAI API endpoint used by the batch. | 
**errors** | Option<[**models::BatchErrors**](Batch_errors.md)> |  | [optional]
**input_file_id** | **String** | The ID of the input file for the batch. | 
**completion_window** | **String** | The time frame within which the batch should be processed. | 
**status** | **String** | The current status of the batch. | 
**output_file_id** | Option<**String**> | The ID of the file containing the outputs of successfully executed requests. | [optional]
**error_file_id** | Option<**String**> | The ID of the file containing the outputs of requests with errors. | [optional]
**created_at** | **i32** | The Unix timestamp (in seconds) for when the batch was created. | 
**in_progress_at** | Option<**i32**> | The Unix timestamp (in seconds) for when the batch started processing. | [optional]
**expires_at** | Option<**i32**> | The Unix timestamp (in seconds) for when the batch will expire. | [optional]
**finalizing_at** | Option<**i32**> | The Unix timestamp (in seconds) for when the batch started finalizing. | [optional]
**completed_at** | Option<**i32**> | The Unix timestamp (in seconds) for when the batch was completed. | [optional]
**failed_at** | Option<**i32**> | The Unix timestamp (in seconds) for when the batch failed. | [optional]
**expired_at** | Option<**i32**> | The Unix timestamp (in seconds) for when the batch expired. | [optional]
**cancelling_at** | Option<**i32**> | The Unix timestamp (in seconds) for when the batch started cancelling. | [optional]
**cancelled_at** | Option<**i32**> | The Unix timestamp (in seconds) for when the batch was cancelled. | [optional]
**request_counts** | Option<[**models::BatchRequestCounts**](BatchRequestCounts.md)> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.  Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


