# WebhookResponseIncomplete

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **i32** | The Unix timestamp (in seconds) of when the model response was interrupted.  | 
**id** | **String** | The unique ID of the event.  | 
**data** | [**models::WebhookResponseCancelledData**](WebhookResponseCancelledData.md) |  | 
**object** | Option<**Object**> | The object of the event. Always `event`.  (enum: event) | [optional]
**r#type** | **Type** | The type of the event. Always `response.incomplete`.  (enum: response.incomplete) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


