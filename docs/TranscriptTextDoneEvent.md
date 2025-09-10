# TranscriptTextDoneEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always `transcript.text.done`.  | 
**text** | **String** | The text that was transcribed.  | 
**logprobs** | Option<[**Vec<models::TranscriptTextDeltaEventLogprobsInner>**](TranscriptTextDeltaEvent_logprobs_inner.md)> | The log probabilities of the individual tokens in the transcription. Only included if you [create a transcription](https://platform.openai.com/docs/api-reference/audio/create-transcription) with the `include[]` parameter set to `logprobs`.  | [optional]
**usage** | Option<[**models::TranscriptTextUsageTokens**](TranscriptTextUsageTokens.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


