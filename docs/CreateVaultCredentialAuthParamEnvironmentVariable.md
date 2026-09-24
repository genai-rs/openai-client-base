# CreateVaultCredentialAuthParamEnvironmentVariable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `environment_variable`. | 
**secret_name** | **String** | The environment variable name that receives the placeholder, such as `SERVICE_API_KEY`. Use ASCII letters, digits, and underscores, starting with a letter or underscore. Names starting with `CODEX_` and managed proxy or certificate variable names are reserved. | 
**secret_value** | **String** | The write-only secret to store. Never returned in credential resources or supplied directly to sandbox code. Must be nonempty and must not contain carriage returns, newlines, or NUL bytes. | 
**networking** | [**models::VaultCredentialNetworkingParam**](VaultCredentialNetworkingParam.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


