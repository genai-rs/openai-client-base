# BetaResponseAudioTranscriptDeltaEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> | The agent that owns this multi-agent streaming event. | [optional]
**r#type** | **Type** | The type of the event. Always `response.audio.transcript.delta`.  (enum: response.audio.transcript.delta) | 
**delta** | **String** | The partial transcript of the audio response.  | 
**sequence_number** | **i32** | The sequence number of this event. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


