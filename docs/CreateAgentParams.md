# CreateAgentParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<**std::collections::HashMap<String, String>**> | Up to 16 string key-value pairs, with keys up to 64 and values up to 512 characters. Omission or null defaults to an empty map. | [optional]
**name** | Option<**String**> | A human-readable name for the agent. Omission or null leaves the agent unnamed. | [optional]
**model** | **String** | The model to use for the agent. The requested model name is preserved. | 
**reasoning** | Option<[**models::ReasoningParam**](ReasoningParam.md)> | Configuration for model reasoning. Omission uses the model's default effort. | [optional]
**text** | Option<[**models::TextParam**](TextParam.md)> | Configuration for generated text. Defaults to the `text` format and medium verbosity. | [optional]
**service_tier** | Option<[**models::ServiceTierParam**](ServiceTierParam.md)> | The service tier used for model requests. Defaults to `auto`. | [optional]
**instructions** | Option<**String**> | Additional instructions appended to the agent's default base instructions. Omit or set to null to add no custom instructions. | [optional]
**tools** | Option<[**Vec<models::PersistedAgentToolConfigParam>**](PersistedAgentToolConfigParam.md)> | Tools available to the agent. Defaults to an empty list. The tool list must fit within 3 MiB (3,145,728 bytes) of compact UTF-8 JSON. | [optional]
**multi_agent** | Option<[**models::MultiAgentConfigCurrentParam**](MultiAgentConfigCurrentParam.md)> | Configuration for creating and coordinating subagents. Subagent tools are disabled by default. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


