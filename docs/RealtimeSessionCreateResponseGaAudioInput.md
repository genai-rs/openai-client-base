# RealtimeSessionCreateResponseGaAudioInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**format** | Option<[**models::RealtimeAudioFormats**](RealtimeAudioFormats.md)> |  | [optional]
**transcription** | Option<[**models::AudioTranscriptionResponse**](AudioTranscriptionResponse.md)> | Configuration for input audio transcription, defaults to off and can be set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs asynchronously through [the /audio/transcriptions endpoint](https://developers.openai.com/api/reference/resources/audio/subresources/transcriptions/methods/create) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.  | [optional]
**noise_reduction** | Option<[**models::RealtimeSessionCreateResponseGaAudioInputNoiseReduction**](RealtimeSessionCreateResponseGAAudioInputNoiseReduction.md)> |  | [optional]
**turn_detection** | Option<[**models::RealtimeTurnDetection**](RealtimeTurnDetection.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


