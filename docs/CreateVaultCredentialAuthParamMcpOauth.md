# CreateVaultCredentialAuthParamMcpOauth

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `mcp_oauth`. | 
**mcp_server_url** | **String** | The HTTPS MCP server URL authorized by this credential. | 
**access_token** | **String** | A write-only OAuth access token; never returned by credential resources. | 
**expires_at** | Option<**String**> | When the OAuth access token expires, as an RFC 3339 timestamp, if known. | [optional]
**refresh** | Option<[**models::CreateMcpOauthRefreshParam**](CreateMcpOauthRefreshParam.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


