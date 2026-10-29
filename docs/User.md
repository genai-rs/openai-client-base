# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | The object type, which is always `organization.user` (enum: organization.user) | 
**id** | **String** | The identifier, which can be referenced in API endpoints | 
**name** | Option<**String**> | The name of the user | [optional]
**email** | Option<**String**> | The email address of the user | [optional]
**role** | Option<**String**> | `owner` or `reader` | [optional]
**added_at** | **i32** | The Unix timestamp (in seconds) of when the user was added. | 
**is_default** | Option<**bool**> | Whether this is the organization's default user. | [optional]
**created** | Option<**i32**> | The Unix timestamp (in seconds) of when the user was created. | [optional]
**user** | Option<[**models::UserUser**](UserUser.md)> |  | [optional]
**is_service_account** | Option<**bool**> | Whether the user is a service account. | [optional]
**is_scale_tier_authorized_purchaser** | Option<**bool**> | Whether the user is an authorized purchaser for Scale Tier. | [optional]
**is_scim_managed** | Option<**bool**> | Whether the user is managed through SCIM. | [optional]
**api_key_last_used_at** | Option<**i32**> | The Unix timestamp (in seconds) of the user's last API key usage. | [optional]
**technical_level** | Option<**String**> | The technical level metadata for the user. | [optional]
**developer_persona** | Option<**String**> | The developer persona metadata for the user. | [optional]
**projects** | Option<[**models::Object024**](Object024.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


