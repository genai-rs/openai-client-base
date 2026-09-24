# ResponseAudioWsDelta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always `response.audio.delta`.  | 
**sequence_number** | **i32** | A sequence number for this chunk of the stream response.  | 
**delta** | **String** | A chunk of Base64 encoded response audio bytes.  | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the originating `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


