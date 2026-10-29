# AgentToolResourceMcp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `mcp`. (enum: mcp) | 
**server_label** | **String** | A label used to identify the MCP server in tool calls. | 
**credential_id** | Option<**String**> | The attached vault credential selected for this MCP server, if any. Optional when exactly one attached credential matches the server URL. | 
**transport** | [**models::McpTransportResource**](McpTransportResource.md) |  | 
**request_metadata** | **std::collections::HashMap<String, serde_json::Value>** | Metadata included with requests to this MCP server. | 
**allowed_tools** | Option<**Vec<String>**> | The MCP tools the agent may call. | 
**required** | **bool** | Whether this MCP server must initialize before the first turn. | 
**connection_origin** | [**models::McpConnectionOriginResource**](McpConnectionOriginResource.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


