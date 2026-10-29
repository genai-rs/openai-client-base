# AgentToolConfigParamWebSearch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `web_search`. (enum: web_search) | 
**mode** | Option<[**models::WebSearchModeParam**](WebSearchModeParam.md)> | The source used for web search results. Defaults to `live`. | [optional]
**context_size** | Option<[**models::WebSearchContextSizeParam**](WebSearchContextSizeParam.md)> | The amount of search context made available to the model. Defaults to `medium`. | [optional]
**allowed_domains** | Option<**Vec<String>**> | Domains the search may include. | [optional]
**location** | Option<[**models::WebSearchLocationParam**](WebSearchLocationParam.md)> | Approximate location used to localize search results. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


