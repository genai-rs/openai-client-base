# RealtimeTranslationServerEventSessionOutputAudioDelta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the server event. | 
**r#type** | **Type** | The event type, must be `session.output_audio.delta`. (enum: session.output_audio.delta) | 
**delta** | **String** | Base64-encoded translated audio data. | 
**sample_rate** | Option<**i64**> | Sample rate of the audio delta.  Defaults to `24000`. | [optional]
**channels** | Option<**i64**> | Number of audio channels.  Defaults to `1`. | [optional]
**format** | Option<**Format**> | Audio encoding for `delta`. (enum: pcm16) | [optional]
**elapsed_ms** | Option<**i64**> | Timing metadata for stream alignment, derived from the translation frame when available. Treat `elapsed_ms` as alignment metadata, not a unique event identifier.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


