# BetaResponseWsError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**r#type** | **String** | The type of the event. Always `error`. | 
**status** | Option<**i32**> | The HTTP status code associated with a WebSocket protocol error. | [optional]
**sequence_number** | Option<**i32**> | The sequence number of an error emitted by the response stream. | [optional]
**error** | [**models::BetaErrorPayload**](BetaErrorPayload.md) |  | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the originating `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


