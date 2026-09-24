# LiveOutputAudioDelta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The event type, always `session.output_audio.delta`. | 
**delta** | **String** | Base64-encoded raw audio. Primary WebSocket events use the session's configured format; reflected sideband events use mono PCM16LE at 24 kHz. | 
**start_ms** | Option<**i32**> | Inclusive session-relative start in milliseconds. Required on reflected sideband events; omitted on the primary WebSocket. | [optional]
**end_ms** | Option<**i32**> | Exclusive session-relative end in milliseconds. Required on reflected sideband events; omitted on the primary WebSocket. Dropped output frames leave gaps between reflected ranges. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


