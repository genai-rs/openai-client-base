# RealtimeResponseCreateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**output_modalities** | Option<**Vec<String>**> | The set of modalities the model used to respond, currently the only possible values are `[\\\"audio\\\"]`, `[\\\"text\\\"]`. Audio output always include a text transcript. Setting the output to mode `text` will disable audio output from the model.  | [optional]
**instructions** | Option<**String**> | The default system instructions (i.e. system message) prepended to model calls. This field allows the client to guide the model on desired responses. The model can be instructed on response content and format, (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion into your voice\", \"laugh frequently\"). The instructions are not guaranteed to be followed by the model, but they provide guidance to the model on the desired behavior. Note that the server sets default instructions which will be used if this field is not set and are visible in the `session.created` event at the start of the session.  | [optional]
**audio** | Option<[**models::RealtimeResponseCreateParamsAudio**](RealtimeResponseCreateParams_audio.md)> |  | [optional]
**tools** | Option<[**Vec<models::RealtimeResponseCreateParamsToolsInner>**](RealtimeResponseCreateParams_tools_inner.md)> | Tools available to the model. | [optional]
**tool_choice** | Option<[**models::RealtimeBetaResponseCreateParamsToolChoice**](RealtimeBetaResponseCreateParams_tool_choice.md)> |  | [optional]
**max_output_tokens** | Option<[**models::RealtimeBetaResponseCreateParamsMaxOutputTokens**](RealtimeBetaResponseCreateParams_max_output_tokens.md)> |  | [optional]
**conversation** | Option<**String**> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.  Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | [optional]
**prompt** | Option<[**models::Prompt**](Prompt.md)> |  | [optional]
**input** | Option<[**Vec<models::RealtimeConversationItem>**](RealtimeConversationItem.md)> | Input items to include in the prompt for the model. Using this field creates a new context for this Response instead of using the default conversation. An empty array `[]` will clear the context for this Response. Note that this can include references to items that previously appeared in the session using their id.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


