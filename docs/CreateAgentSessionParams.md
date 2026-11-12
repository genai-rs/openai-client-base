# CreateAgentSessionParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**environment** | [**models::EnvironmentParam**](EnvironmentParam.md) |  | 
**review** | Option<[**models::SessionReviewParam**](SessionReviewParam.md)> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | Up to 16 string key-value pairs, with keys up to 64 and values up to 512 characters. Omission or null defaults to an empty map. | [optional]
**agent** | Option<[**models::SessionAgentConfigParam**](SessionAgentConfigParam.md)> |  | [optional]
**agent_id** | Option<**String**> | The ID of a saved reusable agent. Omit `agent` to use its configuration unchanged. | [optional]
**vault_ids** | Option<**Vec<String>**> | The IDs of vaults made available to the session. | [optional]
**input** | Option<[**models::CreateSessionInputParam**](CreateSessionInputParam.md)> | Initial input to submit when the session is created. A string is shorthand for a single user message. Required when `environment.type` is `none`, or when `stream` is `true` for an environment that is not `self_hosted`; optional for self-hosted and non-streaming execution environments. | [optional]
**prompt_cache_options** | Option<[**models::PromptCacheOptionsParam2**](PromptCacheOptionsParam2.md)> | Default prompt cache lifetime for turns in this session. Omit to use the model default. | [optional]
**stream** | Option<**bool**> | Whether to stream session events as server-sent events. Defaults to `false`. | [optional]
**stream_options** | Option<[**models::SessionStreamOptionsParam**](SessionStreamOptionsParam.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


