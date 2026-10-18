# CreateAgentSessionParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<**std::collections::HashMap<String, String>**> | Up to 16 string key-value pairs, with keys up to 64 and values up to 512 characters. Omission or null defaults to an empty map. | [optional]
**agent** | Option<[**models::SessionAgentConfigParam**](SessionAgentConfigParam.md)> |  | [optional]
**agent_id** | Option<**String**> | The ID of a saved reusable agent. Omit `agent` to use its configuration unchanged. | [optional]
**environment** | [**models::EnvironmentParam**](EnvironmentParam.md) |  | 
**vault_ids** | Option<**Vec<String>**> | The IDs of vaults made available to the session. | [optional]
**input** | Option<[**models::CreateSessionInputParam**](CreateSessionInputParam.md)> |  | [optional]
**stream** | Option<**bool**> | Whether to stream session events as server-sent events. Defaults to `false`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


