# AdminApiKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | The object type, which is always `organization.admin_api_key` | 
**id** | **String** | The identifier, which can be referenced in API endpoints | 
**name** | **String** | The name of the API key | 
**redacted_value** | **String** | The redacted value of the API key | 
**value** | Option<**String**> | The value of the API key. Only shown on create. | [optional]
**created_at** | **i64** | The Unix timestamp (in seconds) of when the API key was created | 
**last_used_at** | **i64** | The Unix timestamp (in seconds) of when the API key was last used | 
**owner** | [**models::AdminApiKeyOwner**](AdminApiKey_owner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


