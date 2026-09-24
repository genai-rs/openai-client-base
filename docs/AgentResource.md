# AgentResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the reusable agent. | 
**object** | **String** | The object type. Always `agent`. | 
**created_at** | **i64** | The Unix timestamp, in seconds, when the agent was created. | 
**updated_at** | **i64** | The Unix timestamp, in seconds, when the agent was last updated. | 
**name** | Option<**String**> | A human-readable name for the agent, or null if it is unnamed. | 
**metadata** | **std::collections::HashMap<String, String>** | Custom string key-value pairs attached to the agent. | 
**model** | **String** | The requested model name used for inference. | 
**reasoning** | [**models::ReasoningResource**](ReasoningResource.md) |  | 
**text** | [**models::TextResource**](TextResource.md) |  | 
**service_tier** | [**models::ServiceTierResource**](ServiceTierResource.md) |  | 
**instructions** | Option<**String**> | Custom instructions appended to the agent's default base instructions. | 
**tools** | [**Vec<models::PersistedAgentToolResource>**](PersistedAgentToolResource.md) | Tools available to the agent. | 
**multi_agent** | [**models::MultiAgentConfigResource**](MultiAgentConfigResource.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


