# SessionAgentResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the agent. | 
**name** | Option<**String**> | The reusable agent's name when the session was created, or null if no name was saved. Later changes to the agent's name do not affect this value. | 
**model** | **String** | The model used by the agent. | 
**reasoning** | [**models::ReasoningResource**](ReasoningResource.md) |  | 
**text** | [**models::TextResource**](TextResource.md) |  | 
**service_tier** | [**models::ServiceTierResource**](ServiceTierResource.md) |  | 
**instructions** | Option<**String**> | Custom instructions appended to the agent's default base instructions. | 
**tools** | [**Vec<models::AgentToolResource>**](AgentToolResource.md) | Tools available to the agent. | 
**multi_agent** | [**models::MultiAgentConfigResource**](MultiAgentConfigResource.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


