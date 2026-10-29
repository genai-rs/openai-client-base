# CreateChatCompletionStreamResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique identifier for the chat completion. Each chunk has the same ID. | 
**choices** | [**Vec<models::CreateChatCompletionStreamResponseChoicesInner>**](CreateChatCompletionStreamResponseChoicesInner.md) | A list of chat completion choices. Can contain more than one elements if `n` is greater than 1. Can also be empty for the last chunk if you set `stream_options: {\"include_usage\": true}`.  | 
**created** | **i32** | The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp. | 
**model** | **String** | The model to generate the completion. | 
**obfuscation** | Option<**String**> | An obfuscation string added to normalize the size of streamed chunks as a mitigation to certain side-channel attacks. The field is included by default and omitted when `stream_options.include_obfuscation` is `false`.  | [optional]
**service_tier** | Option<[**models::ServiceTier**](ServiceTier.md)> |  | [optional]
**system_fingerprint** | Option<**String**> | This fingerprint represents the backend configuration that the model runs with. Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.  | [optional]
**object** | **Object** | The object type, which is always `chat.completion.chunk`. (enum: chat.completion.chunk) | 
**usage** | Option<[**models::CompletionUsage**](CompletionUsage.md)> | An optional field that will only be present when you set `stream_options: {\"include_usage\": true}` in your request. When present, it contains a null value **except for the last chunk** which contains the token usage statistics for the entire request.  **NOTE:** If the stream is interrupted or cancelled, you may not receive the final usage chunk which contains the total token usage for the request.  | [optional]
**moderation** | Option<[**models::ChatCompletionModeration**](ChatCompletionModeration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


