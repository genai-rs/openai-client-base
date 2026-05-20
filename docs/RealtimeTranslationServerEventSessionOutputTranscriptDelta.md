# RealtimeTranslationServerEventSessionOutputTranscriptDelta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the server event. | 
**r#type** | **String** | The event type, must be `session.output_transcript.delta`. | 
**delta** | **String** | Append-only transcript text for the translated output audio. | 
**elapsed_ms** | Option<**i32**> | Timing metadata for stream alignment, derived from the translation frame when available. It advances in 200 ms increments, but multiple transcript deltas may share the same `elapsed_ms`. Treat it as alignment metadata, not a unique transcript-delta identifier.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


