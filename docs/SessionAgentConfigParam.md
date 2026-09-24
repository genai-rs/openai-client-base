# SessionAgentConfigParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> | The model to use for the agent. The requested model name is preserved. | [optional]
**reasoning** | Option<[**models::ReasoningParam**](ReasoningParam.md)> |  | [optional]
**text** | Option<[**models::TextParam**](TextParam.md)> |  | [optional]
**service_tier** | Option<[**models::ServiceTierParam**](ServiceTierParam.md)> |  | [optional]
**instructions** | Option<**String**> | Additional instructions appended to the agent's default base instructions. Omit to leave unchanged. | [optional]
**multi_agent** | Option<[**models::MultiAgentConfigCurrentParam**](MultiAgentConfigCurrentParam.md)> |  | [optional]
**tools** | Option<[**Vec<models::AgentToolConfigParam>**](AgentToolConfigParam.md)> | Tools available to the agent. Omit to inherit, or pass null to clear them. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


