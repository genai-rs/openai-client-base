# RealtimeServerEventConversationItemInputAudioTranscriptionDelta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the server event. | 
**r#type** | **String** | The event type, must be `conversation.item.input_audio_transcription.delta`. | 
**item_id** | **String** | The ID of the item containing the audio that is being transcribed. | 
**content_index** | Option<**i32**> | The index of the content part in the item's content array. | [optional]
**delta** | Option<**String**> | The text delta. | [optional]
**logprobs** | Option<[**Vec<models::LogProbProperties>**](LogProbProperties.md)> | The log probabilities of the transcription. These can be enabled by configurating the session with `\"include\": [\"item.input_audio_transcription.logprobs\"]`. Each entry in the array corresponds a log probability of which token would be selected for this chunk of transcription. This can help to identify if it was possible there were multiple valid options for a given chunk of transcription. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


