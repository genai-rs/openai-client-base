# RealtimeBetaServerEventConversationItemInputAudioTranscriptionDelta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the server event. | 
**r#type** | **String** | The event type, must be `conversation.item.input_audio_transcription.delta`. | 
**item_id** | **String** | The ID of the item. | 
**content_index** | Option<**i32**> | The index of the content part in the item's content array. | [optional]
**delta** | Option<**String**> | The text delta. | [optional]
**logprobs** | Option<[**Vec<models::LogProbProperties>**](LogProbProperties.md)> | The log probabilities of the transcription. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


