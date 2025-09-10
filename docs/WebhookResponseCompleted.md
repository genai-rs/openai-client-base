# WebhookResponseCompleted

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **i32** | The Unix timestamp (in seconds) of when the model response was completed.  | 
**id** | **String** | The unique ID of the event.  | 
**data** | [**models::WebhookResponseCancelledData**](WebhookResponseCancelled_data.md) |  | 
**object** | Option<**String**> | The object of the event. Always `event`.  | [optional]
**r#type** | **String** | The type of the event. Always `response.completed`.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


