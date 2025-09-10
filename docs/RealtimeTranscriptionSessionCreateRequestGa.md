# RealtimeTranscriptionSessionCreateRequestGa

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of session to create. Always `transcription` for transcription sessions.  | 
**audio** | Option<[**models::RealtimeTranscriptionSessionCreateRequestGaAudio**](RealtimeTranscriptionSessionCreateRequestGA_audio.md)> |  | [optional]
**include** | Option<**Vec<String>**> | Additional fields to include in server outputs.  `item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


