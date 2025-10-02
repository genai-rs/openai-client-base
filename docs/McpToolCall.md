# McpToolCall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the item. Always `mcp_call`.  | 
**id** | **String** | The unique ID of the tool call.  | 
**server_label** | **String** | The label of the MCP server running the tool.  | 
**name** | **String** | The name of the tool that was run.  | 
**arguments** | **String** | A JSON string of the arguments passed to the tool.  | 
**output** | Option<**String**> | The output from the tool call.  | [optional]
**error** | Option<**String**> | The error from the tool call, if any.  | [optional]
**status** | Option<[**models::McpToolCallStatus**](MCPToolCallStatus.md)> |  | [optional]
**approval_request_id** | Option<**String**> | Unique identifier for the MCP tool call approval request. Include this value in a subsequent `mcp_approval_response` input to approve or reject the corresponding tool call.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


