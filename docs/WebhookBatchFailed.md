# WebhookBatchFailed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **i32** | The Unix timestamp (in seconds) of when the batch API request failed.  | 
**id** | **String** | The unique ID of the event.  | 
**data** | [**models::WebhookBatchCancelledData**](WebhookBatchCancelledData.md) |  | 
**object** | Option<**Object**> | The object of the event. Always `event`.  (enum: event) | [optional]
**r#type** | **Type** | The type of the event. Always `batch.failed`.  (enum: batch.failed) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


