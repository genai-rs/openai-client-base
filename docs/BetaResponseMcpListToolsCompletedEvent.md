# BetaResponseMcpListToolsCompletedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> | The agent that owns this multi-agent streaming event. | [optional]
**r#type** | **Type** | The type of the event. Always 'response.mcp_list_tools.completed'. (enum: response.mcp_list_tools.completed) | 
**item_id** | **String** | The ID of the MCP tool call item that produced this output. | 
**output_index** | **i32** | The index of the output item that was processed. | 
**sequence_number** | **i32** | The sequence number of this event. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


