# RotateVaultCredentialAuthParamMcpOauth

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `mcp_oauth`. (enum: mcp_oauth) | 
**access_token** | Option<**String**> | A write-only replacement OAuth access token. | [optional]
**expires_at** | Option<**String**> | The replacement expiry as an RFC 3339 timestamp, or `null` to clear it. Omitting this field preserves the expiry unless a new access token is supplied, in which case the expiry is cleared. | [optional]
**refresh** | Option<[**models::RotateMcpOauthRefreshParam**](RotateMcpOauthRefreshParam.md)> | Optional write-only refresh-token and client-secret updates. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


