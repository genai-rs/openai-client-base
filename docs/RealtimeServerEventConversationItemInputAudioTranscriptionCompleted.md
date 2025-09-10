# RealtimeServerEventConversationItemInputAudioTranscriptionCompleted

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the server event. | 
**r#type** | **String** | The event type, must be `conversation.item.input_audio_transcription.completed`.  | 
**item_id** | **String** | The ID of the item containing the audio that is being transcribed. | 
**content_index** | **i32** | The index of the content part containing the audio. | 
**transcript** | **String** | The transcribed text. | 
**logprobs** | Option<[**Vec<models::LogProbProperties>**](LogProbProperties.md)> | The log probabilities of the transcription. | [optional]
**usage** | [**models::RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage**](RealtimeServerEventConversationItemInputAudioTranscriptionCompleted_usage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


