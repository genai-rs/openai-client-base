# BetaResponseSteerPendingEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The event discriminator. Always `response.steer.pending`. | 
**sequence_number** | **i32** | The sequence number for this event. | 
**steer** | [**models::ResponseSteerPendingEventSteer**](ResponseSteerPendingEvent_steer.md) |  | 
**reason** | [**models::BetaResponseSteerPendingReason**](BetaResponseSteerPendingReason.md) |  | 
**required_input** | [**Vec<models::BetaResponseSteerRequiredInput>**](BetaResponseSteerRequiredInput.md) | Input stubs identifying outstanding client-owned tool results or approval decisions. Each stub contains identifying fields only; the client supplies the result before including it in `response.create`.  | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the target response's `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


