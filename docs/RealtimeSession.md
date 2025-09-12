# RealtimeSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique identifier for the session that looks like `sess_1234567890abcdef`.  | [optional]
**object** | Option<**String**> | The object type. Always `realtime.session`. | [optional]
**modalities** | Option<[**serde_json::Value**](.md)> |  | [optional]
**model** | Option<**String**> | The Realtime model used for this session.  | [optional]
**instructions** | Option<**String**> | The default system instructions (i.e. system message) prepended to model calls. This field allows the client to guide the model on desired responses. The model can be instructed on response content and format, (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion into your voice\", \"laugh frequently\"). The instructions are not guaranteed to be followed by the model, but they provide guidance to the model on the desired behavior.   Note that the server sets default instructions which will be used if this field is not set and are visible in the `session.created` event at the start of the session.  | [optional]
**voice** | Option<**String**> | The voice the model uses to respond. Voice cannot be changed during the session once the model has responded with audio at least once. Current voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`, `shimmer`, and `verse`.  | [optional]
**input_audio_format** | Option<**String**> | The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`. For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, single channel (mono), and little-endian byte order.  | [optional]
**output_audio_format** | Option<**String**> | The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`. For `pcm16`, output audio is sampled at a rate of 24kHz.  | [optional]
**input_audio_transcription** | Option<[**models::AudioTranscription**](AudioTranscription.md)> | Configuration for input audio transcription, defaults to off and can be set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.  | [optional]
**turn_detection** | Option<[**models::RealtimeTurnDetection**](RealtimeTurnDetection.md)> |  | [optional]
**input_audio_noise_reduction** | Option<[**models::RealtimeSessionInputAudioNoiseReduction**](RealtimeSession_input_audio_noise_reduction.md)> |  | [optional]
**speed** | Option<**f64**> | The speed of the model's spoken response. 1.0 is the default speed. 0.25 is the minimum speed. 1.5 is the maximum speed. This value can only be changed in between model turns, not while a response is in progress.  | [optional]
**tracing** | Option<[**models::TracingConfiguration**](Tracing_Configuration.md)> |  | [optional]
**tools** | Option<[**Vec<models::RealtimeFunctionTool>**](RealtimeFunctionTool.md)> | Tools (functions) available to the model. | [optional]
**tool_choice** | Option<**String**> | How the model chooses tools. Options are `auto`, `none`, `required`, or specify a function.  | [optional]
**temperature** | Option<**f64**> | Sampling temperature for the model, limited to [0.6, 1.2]. For audio models a temperature of 0.8 is highly recommended for best performance.  | [optional]
**max_response_output_tokens** | Option<[**models::RealtimeBetaResponseCreateParamsMaxOutputTokens**](RealtimeBetaResponseCreateParams_max_output_tokens.md)> |  | [optional]
**expires_at** | Option<**i32**> | Expiration timestamp for the session, in seconds since epoch. | [optional]
**prompt** | Option<[**models::Prompt**](Prompt.md)> |  | [optional]
**include** | Option<**Vec<String>**> | Additional fields to include in server outputs. - `item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


