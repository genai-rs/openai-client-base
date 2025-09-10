# WebhookBatchCancelled

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **i32** | The Unix timestamp (in seconds) of when the batch API request was cancelled.  | 
**id** | **String** | The unique ID of the event.  | 
**data** | [**models::WebhookBatchCancelledData**](WebhookBatchCancelled_data.md) |  | 
**object** | Option<**String**> | The object of the event. Always `event`.  | [optional]
**r#type** | **String** | The type of the event. Always `batch.cancelled`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


