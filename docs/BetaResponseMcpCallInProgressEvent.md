# BetaResponseMcpCallInProgressEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> | The agent that owns this multi-agent streaming event. | [optional]
**r#type** | **Type** | The type of the event. Always 'response.mcp_call.in_progress'. (enum: response.mcp_call.in_progress) | 
**sequence_number** | **i32** | The sequence number of this event. | 
**output_index** | **i32** | The index of the output item in the response's output array. | 
**item_id** | **String** | The unique identifier of the MCP tool call item being processed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


