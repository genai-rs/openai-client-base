# ResponseSteerFailedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The event discriminator. Always `response.steer.failed`. | 
**sequence_number** | **i32** | The sequence number for this event. | 
**steer** | [**models::ResponseSteerFailedEventSteer**](ResponseSteerFailedEvent_steer.md) |  | 
**error** | [**models::ResponseSteerFailedEventError**](ResponseSteerFailedEvent_error.md) |  | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event, when the target response is available and its `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


