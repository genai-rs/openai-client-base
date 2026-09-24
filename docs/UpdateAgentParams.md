# UpdateAgentParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> | The model to use for the agent. The requested model name is preserved. | [optional]
**reasoning** | Option<[**models::ReasoningParam**](ReasoningParam.md)> |  | [optional]
**text** | Option<[**models::TextParam**](TextParam.md)> |  | [optional]
**service_tier** | Option<[**models::ServiceTierParam**](ServiceTierParam.md)> |  | [optional]
**instructions** | Option<**String**> | Additional instructions appended to the agent's default base instructions. Omit to leave unchanged. | [optional]
**multi_agent** | Option<[**models::MultiAgentConfigCurrentParam**](MultiAgentConfigCurrentParam.md)> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | Replaces all metadata. Omit to leave unchanged, or pass null or {} to clear it. Up to 16 string key-value pairs, with keys up to 64 and values up to 512 characters. | [optional]
**name** | Option<**String**> | A replacement name. Omit to leave unchanged, or pass null to clear it. | [optional]
**tools** | Option<[**Vec<models::PersistedAgentToolConfigParam>**](PersistedAgentToolConfigParam.md)> | Tools available to the agent. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


