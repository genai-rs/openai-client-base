# RealtimeTranslationServerEventSessionOutputAudioDelta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the server event. | 
**r#type** | **String** | The event type, must be `session.output_audio.delta`. | 
**delta** | **String** | Base64-encoded translated audio data. | 
**sample_rate** | Option<**i32**> | Sample rate of the audio delta. | [optional]
**channels** | Option<**i32**> | Number of audio channels. | [optional]
**format** | Option<**String**> | Audio encoding for `delta`. | [optional]
**elapsed_ms** | Option<**i32**> | Timing metadata for stream alignment, derived from the translation frame when available. Treat `elapsed_ms` as alignment metadata, not a unique event identifier.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


