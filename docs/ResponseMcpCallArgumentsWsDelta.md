# ResponseMcpCallArgumentsWsDelta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always 'response.mcp_call_arguments.delta'. | 
**output_index** | **i32** | The index of the output item in the response's output array. | 
**item_id** | **String** | The unique identifier of the MCP tool call item being processed. | 
**delta** | **String** | A JSON string containing the partial update to the arguments for the MCP tool call.  | 
**sequence_number** | **i32** | The sequence number of this event. | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the originating `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


