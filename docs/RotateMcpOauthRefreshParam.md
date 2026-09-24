# RotateMcpOauthRefreshParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**refresh_token** | Option<**String**> | The replacement refresh token. Omit or pass `null` to keep the stored token. This secret is never returned in resources. | [optional]
**scope** | Option<**String**> | Replacement space-separated OAuth scopes for refresh requests. Omit to keep the scopes, or pass `null` to stop sending a scope parameter. | [optional]
**token_endpoint_auth** | Option<[**models::RotateMcpOauthTokenEndpointAuthParam**](RotateMcpOauthTokenEndpointAuthParam.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


