# CreateTranscription200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** | The transcribed text. | 
**logprobs** | Option<[**Vec<models::CreateTranscriptionResponseJsonLogprobsInner>**](CreateTranscriptionResponseJson_logprobs_inner.md)> | The log probabilities of the tokens in the transcription. Only returned with the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe` if `logprobs` is added to the `include` array.  | [optional]
**usage** | Option<[**models::TranscriptTextUsageDuration**](TranscriptTextUsageDuration.md)> |  | [optional]
**language** | **String** | The language of the input audio. | 
**duration** | **f64** | The duration of the input audio. | 
**words** | Option<[**Vec<models::TranscriptionWord>**](TranscriptionWord.md)> | Extracted words and their corresponding timestamps. | [optional]
**segments** | Option<[**Vec<models::TranscriptionSegment>**](TranscriptionSegment.md)> | Segments of the transcribed text and their corresponding details. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


