# WebhookFineTuningJobFailed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **i32** | The Unix timestamp (in seconds) of when the fine-tuning job failed.  | 
**id** | **String** | The unique ID of the event.  | 
**data** | [**models::WebhookFineTuningJobCancelledData**](WebhookFineTuningJobCancelledData.md) |  | 
**object** | Option<**Object**> | The object of the event. Always `event`.  (enum: event) | [optional]
**r#type** | **Type** | The type of the event. Always `fine_tuning.job.failed`.  (enum: fine_tuning.job.failed) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


