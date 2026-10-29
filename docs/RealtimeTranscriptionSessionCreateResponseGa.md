# RealtimeTranscriptionSessionCreateResponseGa

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of session. Always `transcription` for transcription sessions.  (enum: transcription) | 
**id** | **String** | Unique identifier for the session that looks like `sess_1234567890abcdef`.  | 
**object** | **String** | The object type. Always `realtime.transcription_session`. | 
**expires_at** | Option<**i32**> | Expiration timestamp for the session, in seconds since epoch. | [optional]
**include** | Option<**Vec<Include>**> | Additional fields to include in server outputs. - `item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.  (enum: item.input_audio_transcription.logprobs) | [optional]
**audio** | Option<[**models::RealtimeTranscriptionSessionCreateResponseGaAudio**](RealtimeTranscriptionSessionCreateResponseGAAudio.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


