# CreateTranscription200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** | The transcribed text. | 
**languages** | Option<[**Vec<models::TranscriptionLanguage>**](TranscriptionLanguage.md)> | The languages detected in the audio. Returned by `gpt-transcribe`. An empty array indicates that no language could be reliably detected.  | [optional]
**logprobs** | Option<[**Vec<models::CreateTranscriptionResponseJsonLogprobsInner>**](CreateTranscriptionResponseJson_logprobs_inner.md)> | The log probabilities of the tokens in the transcription. Only returned with the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe` if `logprobs` is added to the `include` array.  | [optional]
**usage** | Option<[**models::TranscriptTextUsageDuration**](TranscriptTextUsageDuration.md)> |  | [optional]
**task** | **String** | The type of task that was run. Always `transcribe`. | 
**duration** | **f64** | The duration of the input audio. | 
**segments** | [**Vec<models::TranscriptionSegment>**](TranscriptionSegment.md) | Segments of the transcribed text and their corresponding details. | 
**language** | **String** | The language of the input audio. | 
**words** | Option<[**Vec<models::TranscriptionWord>**](TranscriptionWord.md)> | Extracted words and their corresponding timestamps. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


