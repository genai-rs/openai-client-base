# RealtimeSessionCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_secret** | Option<[**models::RealtimeSessionCreateResponseClientSecret**](RealtimeSessionCreateResponse_client_secret.md)> |  | [optional]
**r#type** | Option<**String**> | The type of session to create. Always `realtime` for the Realtime API.  | [optional]
**output_modalities** | Option<**Vec<String>**> | The set of modalities the model can respond with. It defaults to `[\"audio\"]`, indicating that the model will respond with audio plus a transcript. `[\"text\"]` can be used to make the model respond with text only. It is not possible to request both `text` and `audio` at the same time.  | [optional]
**model** | Option<**String**> | The Realtime model used for this session.  | [optional]
**instructions** | Option<**String**> | The default system instructions (i.e. system message) prepended to model calls. This field allows the client to guide the model on desired responses. The model can be instructed on response content and format, (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion into your voice\", \"laugh frequently\"). The instructions are not guaranteed to be followed by the model, but they provide guidance to the model on the desired behavior.  Note that the server sets default instructions which will be used if this field is not set and are visible in the `session.created` event at the start of the session.  | [optional]
**audio** | Option<[**models::RealtimeSessionCreateRequestGaAudio**](RealtimeSessionCreateRequestGA_audio.md)> |  | [optional]
**include** | Option<**Vec<String>**> | Additional fields to include in server outputs.  `item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.  | [optional]
**tracing** | Option<[**models::TracingConfiguration2**](Tracing_Configuration_2.md)> |  | [optional]
**tools** | Option<[**Vec<models::RealtimeResponseCreateParamsToolsInner>**](RealtimeResponseCreateParams_tools_inner.md)> | Tools available to the model. | [optional]
**tool_choice** | Option<[**models::RealtimeBetaResponseCreateParamsToolChoice**](RealtimeBetaResponseCreateParams_tool_choice.md)> |  | [optional]
**max_output_tokens** | Option<[**models::RealtimeBetaResponseCreateParamsMaxOutputTokens**](RealtimeBetaResponseCreateParams_max_output_tokens.md)> |  | [optional]
**truncation** | Option<[**models::RealtimeTruncation**](RealtimeTruncation.md)> |  | [optional]
**prompt** | Option<[**models::Prompt**](Prompt.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


