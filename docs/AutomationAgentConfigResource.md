# AutomationAgentConfigResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | **String** | The requested model name. | 
**reasoning** | [**models::ReasoningResource**](ReasoningResource.md) |  | 
**text** | [**models::TextResource**](TextResource.md) |  | 
**service_tier** | [**models::ServiceTierResource**](ServiceTierResource.md) |  | 
**instructions** | Option<**String**> | Custom instructions, or null when absent. | 
**tools** | [**Vec<models::AgentToolResource>**](AgentToolResource.md) | Configured tools, excluding confidential credentials. | 
**multi_agent** | [**models::MultiAgentConfigResource**](MultiAgentConfigResource.md) |  | 
**compaction** | Option<[**models::CompactionResource**](CompactionResource.md)> |  | [optional]
**experimental** | Option<[**models::ExperimentalConfigResource**](ExperimentalConfigResource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


