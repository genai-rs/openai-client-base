# CreateTranscriptionResponseJson

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** | The transcribed text. | 
**logprobs** | Option<[**Vec<models::CreateTranscriptionResponseJsonLogprobsInner>**](CreateTranscriptionResponseJson_logprobs_inner.md)> | The log probabilities of the tokens in the transcription. Only returned with the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe` if `logprobs` is added to the `include` array.  | [optional]
**usage** | Option<[**models::CreateTranscriptionResponseJsonUsage**](CreateTranscriptionResponseJson_usage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


