# RealtimeSessionCreateResponseGa

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of session to create. Always `realtime` for the Realtime API.  | 
**id** | **String** | Unique identifier for the session that looks like `sess_1234567890abcdef`.  | 
**object** | **String** | The object type. Always `realtime.session`. | 
**expires_at** | Option<**i32**> | Expiration timestamp for the session, in seconds since epoch. | [optional]
**output_modalities** | Option<**Vec<String>**> | The set of modalities the model can respond with. It defaults to `[\"audio\"]`, indicating that the model will respond with audio plus a transcript. `[\"text\"]` can be used to make the model respond with text only. It is not possible to request both `text` and `audio` at the same time.  | [optional]
**model** | Option<**String**> | The Realtime model used for this session.  | [optional]
**instructions** | Option<**String**> | The default system instructions (i.e. system message) prepended to model calls. This field allows the client to guide the model on desired responses. The model can be instructed on response content and format, (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion into your voice\", \"laugh frequently\"). The instructions are not guaranteed to be followed by the model, but they provide guidance to the model on the desired behavior.  Note that the server sets default instructions which will be used if this field is not set and are visible in the `session.created` event at the start of the session.  | [optional]
**audio** | Option<[**models::RealtimeSessionCreateResponseGaAudio**](RealtimeSessionCreateResponseGA_audio.md)> |  | [optional]
**include** | Option<**Vec<String>**> | Additional fields to include in server outputs.  `item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.  | [optional]
**tracing** | Option<[**models::TracingConfiguration1**](Tracing_Configuration_1.md)> |  | [optional]
**tools** | Option<[**Vec<models::RealtimeResponseCreateParamsToolsInner>**](RealtimeResponseCreateParams_tools_inner.md)> | Tools available to the model. | [optional]
**tool_choice** | Option<[**models::RealtimeBetaResponseCreateParamsToolChoice**](RealtimeBetaResponseCreateParams_tool_choice.md)> |  | [optional]
**reasoning** | Option<[**models::RealtimeReasoning**](RealtimeReasoning.md)> |  | [optional]
**max_output_tokens** | Option<[**models::RealtimeBetaResponseCreateParamsMaxOutputTokens**](RealtimeBetaResponseCreateParams_max_output_tokens.md)> |  | [optional]
**truncation** | Option<[**models::RealtimeTruncation**](RealtimeTruncation.md)> |  | [optional]
**prompt** | Option<[**models::Prompt**](Prompt.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


