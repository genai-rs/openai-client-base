# LiveInputAudioAppendEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | Option<**String**> | Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id. | [optional]
**r#type** | **String** | The Live client event type. Always `session.input_audio.append`. | 
**audio** | **String** | Base64-encoded raw audio in the startup-selected format, without a WAV or other container header. Primary WebSocket only; media transports use their audio track. Audio appends have no acknowledgment. Reflected sideband server events reuse this event type and audio key, with no timestamps or event_id; their audio is always mono PCM16LE at 24 kHz. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


