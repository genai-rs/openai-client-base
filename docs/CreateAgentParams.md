# CreateAgentParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<**std::collections::HashMap<String, String>**> | Up to 16 string key-value pairs, with keys up to 64 and values up to 512 characters. Omission or null defaults to an empty map. | [optional]
**name** | Option<**String**> | A human-readable name for the agent. Omission or null leaves the agent unnamed. | [optional]
**model** | **String** | The model to use for the agent. The requested model name is preserved. | 
**reasoning** | Option<[**models::ReasoningParam**](ReasoningParam.md)> |  | [optional]
**text** | Option<[**models::TextParam**](TextParam.md)> |  | [optional]
**service_tier** | Option<[**models::ServiceTierParam**](ServiceTierParam.md)> |  | [optional]
**instructions** | Option<**String**> | Additional instructions appended to the agent's default base instructions. Omit or set to null to add no custom instructions. | [optional]
**tools** | Option<[**Vec<models::PersistedAgentToolConfigParam>**](PersistedAgentToolConfigParam.md)> | Tools available to the agent. Defaults to an empty list. | [optional]
**multi_agent** | Option<[**models::MultiAgentConfigCurrentParam**](MultiAgentConfigCurrentParam.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


