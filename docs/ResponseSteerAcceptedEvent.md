# ResponseSteerAcceptedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The event discriminator. Always `response.steer.accepted`. (enum: response.steer.accepted) | 
**sequence_number** | **i32** | The sequence number for this event. | 
**steer** | [**models::ResponseSteerAcceptedEventSteer**](ResponseSteerAcceptedEventSteer.md) |  | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the target response's `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


