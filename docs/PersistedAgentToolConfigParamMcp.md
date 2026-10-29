# PersistedAgentToolConfigParamMcp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `mcp`. (enum: mcp) | 
**server_label** | **String** | A label used to identify the MCP server in tool calls. | 
**credential_id** | Option<**String**> | The vault credential selected for this MCP server. Optional when exactly one attached credential matches the server URL. | [optional]
**transport** | [**models::PersistedMcpTransportConfigParam**](PersistedMcpTransportConfigParam.md) |  | 
**request_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata included with requests to this MCP server. | [optional]
**allowed_tools** | Option<**Vec<String>**> | The MCP tools the agent may call. All server tools are allowed when omitted. | [optional]
**required** | Option<**bool**> | Whether this MCP server must initialize before the first turn. Defaults to `false`. | [optional]
**connection_origin** | Option<[**models::McpConnectionOriginParam**](McpConnectionOriginParam.md)> | Selects where outbound MCP HTTP connections originate. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


