# WebhookEvalRunFailed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **i32** | The Unix timestamp (in seconds) of when the eval run failed.  | 
**id** | **String** | The unique ID of the event.  | 
**data** | [**models::WebhookEvalRunCanceledData**](WebhookEvalRunCanceledData.md) |  | 
**object** | Option<**Object**> | The object of the event. Always `event`.  (enum: event) | [optional]
**r#type** | **Type** | The type of the event. Always `eval.run.failed`.  (enum: eval.run.failed) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


