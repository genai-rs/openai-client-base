# BetaResponseCustomToolCallInputWsDelta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**r#type** | **String** | The event type identifier. | 
**sequence_number** | **i32** | The sequence number of this event. | 
**output_index** | **i32** | The index of the output this delta applies to. | 
**item_id** | **String** | Unique identifier for the API item associated with this event. | 
**delta** | **String** | The incremental input data (delta) for the custom tool call. | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the originating `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


