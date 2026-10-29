# RealtimeTranscriptionSessionCreateRequestGa

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of session to create. Always `transcription` for transcription sessions.  (enum: transcription) | 
**audio** | Option<[**models::RealtimeTranscriptionSessionCreateRequestGaAudio**](RealtimeTranscriptionSessionCreateRequestGAAudio.md)> |  | [optional]
**include** | Option<**Vec<Include>**> | Additional fields to include in server outputs.  `item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.  (enum: item.input_audio_transcription.logprobs) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


