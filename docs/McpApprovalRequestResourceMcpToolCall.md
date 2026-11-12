# McpApprovalRequestResourceMcpToolCall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `mcp_tool_call`.  Defaults to `\"mcp_tool_call\"`. (enum: mcp_tool_call) | 
**server_label** | **String** |  | 
**name** | Option<**String**> | The tool name, or null when CCA does not provide it. | 
**arguments** | Option<**serde_json::Value**> | The tool arguments as JSON, or null when CCA does not provide them. | 
**message** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


