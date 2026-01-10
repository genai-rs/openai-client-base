# RealtimeSessionCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_secret** | [**models::RealtimeSessionCreateRequestClientSecret**](RealtimeSessionCreateRequest_client_secret.md) |  | 
**modalities** | Option<[**serde_json::Value**](.md)> |  | [optional]
**instructions** | Option<**String**> | The default system instructions (i.e. system message) prepended to model calls. This field allows the client to guide the model on desired responses. The model can be instructed on response content and format, (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion into your voice\", \"laugh frequently\"). The instructions are not guaranteed to be followed by the model, but they provide guidance to the model on the desired behavior. Note that the server sets default instructions which will be used if this field is not set and are visible in the `session.created` event at the start of the session.  | [optional]
**voice** | Option<**String**> | The voice the model uses to respond. Supported built-in voices are `alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`, `shimmer`, `verse`, `marin`, and `cedar`. Voice cannot be changed during the session once the model has responded with audio at least once. | [optional]
**input_audio_format** | Option<**String**> | The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.  | [optional]
**output_audio_format** | Option<**String**> | The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.  | [optional]
**input_audio_transcription** | Option<[**models::RealtimeSessionCreateRequestInputAudioTranscription**](RealtimeSessionCreateRequest_input_audio_transcription.md)> |  | [optional]
**speed** | Option<**f64**> | The speed of the model's spoken response. 1.0 is the default speed. 0.25 is the minimum speed. 1.5 is the maximum speed. This value can only be changed in between model turns, not while a response is in progress.  | [optional]
**tracing** | Option<[**models::TracingConfiguration1**](Tracing_Configuration_1.md)> |  | [optional]
**turn_detection** | Option<[**models::RealtimeSessionCreateRequestTurnDetection**](RealtimeSessionCreateRequest_turn_detection.md)> |  | [optional]
**tools** | Option<[**Vec<models::RealtimeSessionCreateRequestToolsInner>**](RealtimeSessionCreateRequest_tools_inner.md)> | Tools (functions) available to the model. | [optional]
**tool_choice** | Option<**String**> | How the model chooses tools. Options are `auto`, `none`, `required`, or specify a function.  | [optional]
**temperature** | Option<**f64**> | Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.  | [optional]
**max_response_output_tokens** | Option<[**models::RealtimeBetaResponseCreateParamsMaxOutputTokens**](RealtimeBetaResponseCreateParams_max_output_tokens.md)> |  | [optional]
**truncation** | Option<[**models::RealtimeTruncation**](RealtimeTruncation.md)> |  | [optional]
**prompt** | Option<[**models::Prompt**](Prompt.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


