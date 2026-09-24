# PersistedAgentToolResourceMcp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `mcp`. | 
**server_label** | **String** | A label used to identify the MCP server in tool calls. | 
**credential_id** | **String** | The vault credential selected for this MCP server, if any. | 
**transport** | [**models::PersistedMcpTransportResource**](PersistedMcpTransportResource.md) |  | 
**request_metadata** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Metadata included with requests to this MCP server. | 
**allowed_tools** | **Vec<String>** | The MCP tools the agent may call, or null when all server tools are allowed. | 
**required** | **bool** | Whether this MCP server must initialize before the first turn. | 
**connection_origin** | [**models::McpConnectionOriginResource**](McpConnectionOriginResource.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


