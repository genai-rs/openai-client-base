# WebSearchTool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the web search tool. One of `web_search` or `web_search_2025_08_26`. (enum: web_search, web_search_2025_08_26) | 
**external_web_access** | Option<**bool**> | Allow live internet access for web search. Defaults to true when omitted. When false, the web search tool runs in offline/cache-only mode and will not fetch new external content. | [optional]
**filters** | Option<[**models::Object026**](Object026.md)> |  | [optional]
**user_location** | Option<[**models::WebSearchApproximateLocation**](WebSearchApproximateLocation.md)> |  | [optional]
**search_context_size** | Option<**SearchContextSize**> | High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default. (enum: low, medium, high) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


