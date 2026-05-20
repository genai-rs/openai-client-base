# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | The object type, which is always `organization.user` | 
**id** | **String** | The identifier, which can be referenced in API endpoints | 
**name** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**role** | Option<**String**> |  | [optional]
**added_at** | **i32** | The Unix timestamp (in seconds) of when the user was added. | 
**is_default** | Option<**bool**> | Whether this is the organization's default user. | [optional]
**created** | Option<**i32**> | The Unix timestamp (in seconds) of when the user was created. | [optional]
**user** | Option<[**models::UserUser**](User_user.md)> |  | [optional]
**is_service_account** | Option<**bool**> | Whether the user is a service account. | [optional]
**is_scale_tier_authorized_purchaser** | Option<**bool**> |  | [optional]
**is_scim_managed** | Option<**bool**> | Whether the user is managed through SCIM. | [optional]
**api_key_last_used_at** | Option<**i32**> |  | [optional]
**technical_level** | Option<**String**> |  | [optional]
**developer_persona** | Option<**String**> |  | [optional]
**projects** | Option<[**models::Object022**](Object0_22.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


