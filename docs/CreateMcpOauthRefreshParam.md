# CreateMcpOauthRefreshParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token_endpoint** | **String** | The HTTPS OAuth token endpoint used to exchange the refresh token for a new access token. | 
**client_id** | **String** | The OAuth client ID used when requesting a new access token. | 
**resource** | Option<**String**> | The resource URI to send to the OAuth token endpoint during refresh, if required. | [optional]
**scope** | Option<**String**> | Space-separated OAuth scopes to request during refresh, if required. | [optional]
**refresh_token** | **String** | The refresh token to store. This secret is never returned in credential resources. | 
**token_endpoint_auth** | [**models::CreateMcpOauthTokenEndpointAuthParam**](CreateMcpOauthTokenEndpointAuthParam.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


