# RealtimeTranscriptionSessionCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_secret** | [**models::RealtimeTranscriptionSessionCreateResponseClientSecret**](RealtimeTranscriptionSessionCreateResponse_client_secret.md) |  | 
**modalities** | Option<[**serde_json::Value**](.md)> |  | [optional]
**input_audio_format** | Option<**String**> | The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.  | [optional]
**input_audio_transcription** | Option<[**models::AudioTranscription**](AudioTranscription.md)> |  | [optional]
**turn_detection** | Option<[**models::RealtimeSessionCreateRequestTurnDetection**](RealtimeSessionCreateRequest_turn_detection.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


