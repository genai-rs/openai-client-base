# BetaResponseWebSearchCallWsSearching

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**r#type** | **String** | The type of the event. Always `response.web_search_call.searching`.  | 
**output_index** | **i32** | The index of the output item that the web search call is associated with.  | 
**item_id** | **String** | Unique ID for the output item associated with the web search call.  | 
**sequence_number** | **i32** | The sequence number of the web search call being processed. | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the originating `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


