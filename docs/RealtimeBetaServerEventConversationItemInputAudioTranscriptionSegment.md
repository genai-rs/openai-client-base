# RealtimeBetaServerEventConversationItemInputAudioTranscriptionSegment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the server event. | 
**r#type** | **String** | The event type, must be `conversation.item.input_audio_transcription.segment`. | 
**item_id** | **String** | The ID of the item containing the input audio content. | 
**content_index** | **i32** | The index of the input audio content part within the item. | 
**text** | **String** | The text for this segment. | 
**id** | **String** | The segment identifier. | 
**speaker** | **String** | The detected speaker label for this segment. | 
**start** | **f32** | Start time of the segment in seconds. | 
**end** | **f32** | End time of the segment in seconds. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


