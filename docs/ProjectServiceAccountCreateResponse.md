# ProjectServiceAccountCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: organization.project.service_account) | 
**id** | **String** |  | 
**name** | **String** |  | 
**role** | **Role** | Service accounts created with default project membership have role `member`. Accounts created with `create_service_account_only` have role `none`.  (enum: member, none) | 
**created_at** | **i32** |  | 
**api_key** | Option<[**models::ProjectServiceAccountApiKey**](ProjectServiceAccountApiKey.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


