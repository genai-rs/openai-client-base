# RealtimeServerEventInputAudioBufferTimeoutTriggered

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the server event. | 
**r#type** | Option<[**serde_json::Value**](.md)> |  | 
**audio_start_ms** | **i32** | Millisecond offset of audio written to the input audio buffer that was after the playback time of the last model response. | 
**audio_end_ms** | **i32** | Millisecond offset of audio written to the input audio buffer at the time the timeout was triggered. | 
**item_id** | **String** | The ID of the item associated with this segment. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


