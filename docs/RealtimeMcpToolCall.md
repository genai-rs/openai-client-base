# RealtimeMcpToolCall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the item. Always `mcp_call`. | 
**id** | **String** | The unique ID of the tool call. | 
**server_label** | **String** | The label of the MCP server running the tool. | 
**name** | **String** | The name of the tool that was run. | 
**arguments** | **String** | A JSON string of the arguments passed to the tool. | 
**approval_request_id** | Option<**String**> | The ID of an associated approval request, if any. | [optional]
**output** | Option<**String**> | The output from the tool call. | [optional]
**error** | Option<[**models::RealtimeMcpToolCallError**](RealtimeMCPToolCall_error.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


