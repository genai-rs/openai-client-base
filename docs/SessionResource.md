# SessionResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | **std::collections::HashMap<String, String>** | Custom string key-value pairs attached to the session. | 
**id** | **String** | The ID of the session. | 
**object** | **Object** | The object type. Always `agent.session`. (enum: agent.session) | 
**created_at** | **i64** | The Unix timestamp, in seconds, when the session was created. | 
**last_active_at** | **i64** | The Unix timestamp, in seconds, when the session was last active. | 
**status** | [**models::SessionStatusResource**](SessionStatusResource.md) |  | 
**required_actions** | [**Vec<models::SessionRequiredActionResource>**](SessionRequiredActionResource.md) | Actions that must be completed before the session can continue. | 
**error** | Option<**String**> | The error that caused the session to fail, if any. | 
**agent** | [**models::SessionAgentResource**](SessionAgentResource.md) |  | 
**environment** | [**models::EnvironmentResource**](EnvironmentResource.md) |  | 
**vault_ids** | **Vec<String>** | The IDs of vaults made available to the session. | 
**usage** | Option<[**models::TokenUsageResource**](TokenUsageResource.md)> | Best-effort token usage for the session, or null if unknown. Recorded usage may change. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


