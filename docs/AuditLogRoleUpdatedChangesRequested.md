# AuditLogRoleUpdatedChangesRequested

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**role_name** | Option<**String**> | The updated role name, when provided. | [optional]
**resource_id** | Option<**String**> | The resource the role is scoped to. | [optional]
**resource_type** | Option<**String**> | The type of resource the role belongs to. | [optional]
**permissions_added** | Option<**Vec<String>**> | The permissions added to the role. | [optional]
**permissions_removed** | Option<**Vec<String>**> | The permissions removed from the role. | [optional]
**description** | Option<**String**> | The updated role description, when provided. | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Additional metadata stored on the role. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


