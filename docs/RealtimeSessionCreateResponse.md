# RealtimeSessionCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique identifier for the session that looks like `sess_1234567890abcdef`.  | [optional]
**object** | Option<**String**> | The object type. Always `realtime.session`. | [optional]
**expires_at** | Option<**i32**> | Expiration timestamp for the session, in seconds since epoch. | [optional]
**include** | Option<**Vec<String>**> | Additional fields to include in server outputs. - `item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.  | [optional]
**model** | Option<**String**> | The Realtime model used for this session. | [optional]
**output_modalities** | Option<[**serde_json::Value**](.md)> |  | [optional]
**instructions** | Option<**String**> | The default system instructions (i.e. system message) prepended to model calls. This field allows the client to guide the model on desired responses. The model can be instructed on response content and format, (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion into your voice\", \"laugh frequently\"). The instructions are not guaranteed to be followed by the model, but they provide guidance to the model on the desired behavior.  Note that the server sets default instructions which will be used if this field is not set and are visible in the `session.created` event at the start of the session.  | [optional]
**audio** | Option<[**models::RealtimeSessionCreateResponseAudio**](RealtimeSessionCreateResponse_audio.md)> |  | [optional]
**tracing** | Option<[**models::RealtimeSessionCreateResponseTracing**](RealtimeSessionCreateResponseTracing.md)> |  | [optional]
**turn_detection** | Option<[**models::RealtimeSessionCreateRequestTurnDetection**](RealtimeSessionCreateRequest_turn_detection.md)> |  | [optional]
**tools** | Option<[**Vec<models::RealtimeFunctionTool>**](RealtimeFunctionTool.md)> | Tools (functions) available to the model. | [optional]
**tool_choice** | Option<**String**> | How the model chooses tools. Options are `auto`, `none`, `required`, or specify a function.  | [optional]
**max_output_tokens** | Option<[**models::RealtimeBetaResponseCreateParamsMaxOutputTokens**](RealtimeBetaResponseCreateParams_max_output_tokens.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


