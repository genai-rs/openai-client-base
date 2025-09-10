# RealtimeTranscriptionSessionCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**turn_detection** | Option<[**models::RealtimeTranscriptionSessionCreateRequestTurnDetection**](RealtimeTranscriptionSessionCreateRequest_turn_detection.md)> |  | [optional]
**input_audio_noise_reduction** | Option<[**models::RealtimeSessionInputAudioNoiseReduction**](RealtimeSession_input_audio_noise_reduction.md)> |  | [optional]
**input_audio_format** | Option<**String**> | The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`. For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, single channel (mono), and little-endian byte order.  | [optional]
**input_audio_transcription** | Option<[**models::AudioTranscription**](AudioTranscription.md)> |  | [optional]
**include** | Option<**Vec<String>**> | The set of items to include in the transcription. Current available items are: `item.input_audio_transcription.logprobs`  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


