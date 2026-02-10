# TranscriptTextDeltaEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always `transcript.text.delta`.  | 
**delta** | **String** | The text delta that was additionally transcribed.  | 
**logprobs** | Option<[**Vec<models::TranscriptTextDeltaEventLogprobsInner>**](TranscriptTextDeltaEvent_logprobs_inner.md)> | The log probabilities of the delta. Only included if you [create a transcription](/docs/api-reference/audio/create-transcription) with the `include[]` parameter set to `logprobs`.  | [optional]
**segment_id** | Option<**String**> | Identifier of the diarized segment that this delta belongs to. Only present when using `gpt-4o-transcribe-diarize`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


