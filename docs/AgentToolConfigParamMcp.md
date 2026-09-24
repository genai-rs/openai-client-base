# AgentToolConfigParamMcp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `mcp`. | 
**server_label** | **String** | A label used to identify the MCP server in tool calls. | 
**credential_id** | Option<**String**> | The attached vault credential used to authenticate this MCP server. Optional when exactly one attached credential matches the server URL. | [optional]
**transport** | [**models::McpTransportConfigParam**](McpTransportConfigParam.md) |  | 
**request_metadata** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Metadata included with requests to this MCP server. | [optional]
**allowed_tools** | Option<**Vec<String>**> | The MCP tools the agent may call. All server tools are allowed when omitted. | [optional]
**required** | Option<**bool**> | Whether this MCP server must initialize before the first turn. Defaults to `false`. | [optional]
**connection_origin** | Option<[**models::AgentToolConfigParamMcpConnectionOrigin**](AgentToolConfigParamMcp_connection_origin.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


