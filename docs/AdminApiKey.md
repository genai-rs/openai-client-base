# AdminApiKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | The object type, which is always `organization.admin_api_key` (enum: organization.admin_api_key) | 
**id** | **String** | The identifier, which can be referenced in API endpoints | 
**name** | Option<**String**> | The name of the API key | [optional]
**redacted_value** | **String** | The redacted value of the API key | 
**created_at** | **i32** | The Unix timestamp (in seconds) of when the API key was created | 
**expires_at** | Option<**i32**> | The Unix timestamp (in seconds) of when the API key expires | 
**last_used_at** | Option<**i32**> | The Unix timestamp (in seconds) of when the API key was last used | [optional]
**owner** | [**models::AdminApiKeyOwner**](AdminApiKeyOwner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


