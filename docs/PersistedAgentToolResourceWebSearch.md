# PersistedAgentToolResourceWebSearch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `web_search`. (enum: web_search) | 
**mode** | [**models::WebSearchModeResource**](WebSearchModeResource.md) |  | 
**context_size** | [**models::WebSearchContextSizeResource**](WebSearchContextSizeResource.md) |  | 
**allowed_domains** | Option<**Vec<String>**> | Allowed search domains, or `null` when the search is unrestricted. | 
**location** | Option<[**models::WebSearchLocationResource**](WebSearchLocationResource.md)> | Approximate location used to localize search results, if provided. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


