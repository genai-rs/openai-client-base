# AutomationTargetParamNewSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `new_session`.  Defaults to `\"new_session\"`. (enum: new_session) | 
**agent** | Option<[**models::SessionAgentConfigParam**](SessionAgentConfigParam.md)> |  | [optional]
**agent_id** | Option<**String**> | A saved agent ID. Provide exactly one of agent and agent_id. | [optional]
**environment** | [**models::EnvironmentParam**](EnvironmentParam.md) |  | 
**input** | [**Vec<models::InputMessageParam>**](InputMessageParam.md) | Initial user messages submitted with each new session. | 
**vault_ids** | Option<**Vec<String>**> | The IDs of vaults made available to each new session. | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | Metadata attached to each new session. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


