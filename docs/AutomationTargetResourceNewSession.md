# AutomationTargetResourceNewSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `new_session`.  Defaults to `\"new_session\"`. (enum: new_session) | 
**agent_id** | Option<**String**> | The saved agent reference, or null when an inline agent is configured. | 
**agent** | Option<[**models::AutomationAgentConfigResource**](AutomationAgentConfigResource.md)> | The inline agent configuration, or null when agent_id is configured. | 
**environment** | [**models::AutomationEnvironmentResource**](AutomationEnvironmentResource.md) |  | 
**input** | [**Vec<models::InputMessageResource2>**](InputMessageResource2.md) | Initial input delivered to each new session. | 
**vault_ids** | **Vec<String>** | Vaults made available to each new session. | 
**metadata** | **std::collections::HashMap<String, String>** | Metadata attached to each new session. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


