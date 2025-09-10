# McpTool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the MCP tool. Always `mcp`. | 
**server_label** | **String** | A label for this MCP server, used to identify it in tool calls.  | 
**server_url** | Option<**String**> | The URL for the MCP server. One of `server_url` or `connector_id` must be  provided.  | [optional]
**connector_id** | Option<**String**> | Identifier for service connectors, like those available in ChatGPT. One of `server_url` or `connector_id` must be provided. Learn more about service connectors [here](https://platform.openai.com/docs/guides/tools-remote-mcp#connectors).  Currently supported `connector_id` values are:  - Dropbox: `connector_dropbox` - Gmail: `connector_gmail` - Google Calendar: `connector_googlecalendar` - Google Drive: `connector_googledrive` - Microsoft Teams: `connector_microsoftteams` - Outlook Calendar: `connector_outlookcalendar` - Outlook Email: `connector_outlookemail` - SharePoint: `connector_sharepoint`  | [optional]
**authorization** | Option<**String**> | An OAuth access token that can be used with a remote MCP server, either  with a custom MCP server URL or a service connector. Your application must handle the OAuth authorization flow and provide the token here.  | [optional]
**server_description** | Option<**String**> | Optional description of the MCP server, used to provide more context.  | [optional]
**headers** | Option<**std::collections::HashMap<String, String>**> | Optional HTTP headers to send to the MCP server. Use for authentication or other purposes.  | [optional]
**allowed_tools** | Option<[**models::McpToolAllowedTools**](MCPTool_allowed_tools.md)> |  | [optional]
**require_approval** | Option<[**models::McpToolRequireApproval**](MCPTool_require_approval.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


