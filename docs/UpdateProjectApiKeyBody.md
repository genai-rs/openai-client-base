# UpdateProjectApiKeyBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expires_in_seconds** | Option<**i32**> | Number of seconds from this request until the API key expires. Must comply with the organization and project expiry policies. Expired keys cannot be updated. Retrying resets the expiry relative to the new request. Omit to keep the current expiration. | [optional]
**scopes** | Option<**Vec<String>**> | Replace the API key scope restrictions. Omit to leave scopes unchanged. An empty array removes key restrictions (All permissions), subject to the owner's effective permissions. Use [\"api.all.read\"] for read-only access. Scopes do not change the owner's roles or grant administrative permissions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


