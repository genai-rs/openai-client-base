# RealtimeTranscriptionSessionCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_secret** | [**models::RealtimeTranscriptionSessionCreateResponseClientSecret**](RealtimeTranscriptionSessionCreateResponseClientSecret.md) |  | 
**modalities** | Option<**serde_json::Value**> | The set of modalities the model can respond with. To disable audio, set this to [\"text\"].  | [optional]
**input_audio_format** | Option<**String**> | The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.  | [optional]
**input_audio_transcription** | Option<[**models::AudioTranscriptionResponse**](AudioTranscriptionResponse.md)> |  | [optional]
**turn_detection** | Option<[**models::RealtimeSessionCreateRequestTurnDetection**](RealtimeSessionCreateRequestTurnDetection.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


