# CreateTranscriptionResponseDiarizedJson

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task** | **String** | The type of task that was run. Always `transcribe`. | 
**duration** | **f64** | Duration of the input audio in seconds. | 
**text** | **String** | The concatenated transcript text for the entire audio input. | 
**segments** | [**Vec<models::TranscriptionDiarizedSegment>**](TranscriptionDiarizedSegment.md) | Segments of the transcript annotated with timestamps and speaker labels. | 
**usage** | Option<[**models::CreateTranscriptionResponseDiarizedJsonUsage**](CreateTranscriptionResponseDiarizedJson_usage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


