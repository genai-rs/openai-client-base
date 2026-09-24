# ResponseMcpCallWsFailed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always 'response.mcp_call.failed'. | 
**item_id** | **String** | The ID of the MCP tool call item that failed. | 
**output_index** | **i32** | The index of the output item that failed. | 
**sequence_number** | **i32** | The sequence number of this event. | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the originating `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


