# LiveMcpToolInputParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | Defaults to `\"mcp\"`. (enum: mcp) | 
**server_label** | Option<**String**> | A label used to identify the MCP server. | [optional]
**server_url** | Option<**String**> | The URL of the MCP server to connect to. | [optional]
**server_description** | Option<**String**> | An optional human-readable description of the MCP server. | [optional]
**authorization** | Option<**String**> | The authorization token to use when connecting to the MCP server or connector. | [optional]
**headers** | Option<**std::collections::HashMap<String, String>**> | Optional HTTP headers to send when connecting to the MCP server. | [optional]
**allowed_tools** | Option<[**models::LiveMcpToolInputParamAllowedTools**](LiveMcpToolInputParamAllowedTools.md)> |  | [optional]
**require_approval** | Option<[**models::LiveMcpToolInputParamRequireApproval**](LiveMcpToolInputParamRequireApproval.md)> |  | [optional]
**connector_id** | Option<**String**> | The ID of a built-in MCP connector. This field is deprecated for models released after September 1, 2026. Use `server_url` to connect to a remote MCP server, or `tunnel_id` to connect to a local MCP server. | [optional]
**tunnel_id** | Option<**String**> | The Secure MCP Tunnel ID to use instead of a direct server URL. | [optional]
**allowed_callers** | Option<[**Vec<models::LiveCallableToolAllowedCaller>**](LiveCallableToolAllowedCaller.md)> | The tool invocation context(s). | [optional]
**defer_loading** | Option<**bool**> | Whether this MCP tool should be deferred and discovered via tool search. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


