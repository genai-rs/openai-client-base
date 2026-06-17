# AuditLogRoleBoundToResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the resource the role was bound to. ChatGPT workspace connector resources use `<workspace_id>__<connector_id>`. | [optional]
**role_id** | Option<**String**> | The ID of the role that was bound to the resource. | [optional]
**resource_id** | Option<**String**> | The ID of the resource the role was bound to. | [optional]
**resource_type** | Option<**String**> | The type of resource the role was bound to. | [optional]
**permissions** | Option<**Vec<String>**> | The permissions granted to the role for the resource. | [optional]
**workspace_id** | Option<**String**> | The workspace ID for a ChatGPT workspace connector resource. | [optional]
**connector_id** | Option<**String**> | The connector ID for a ChatGPT workspace connector resource. | [optional]
**connector_name** | Option<**String**> | The connector display name for a ChatGPT workspace connector resource, or the connector ID when the display name could not be resolved. | [optional]
**enabled** | Option<**bool**> | Whether the connector is enabled for the role. | [optional]
**source** | Option<**String**> | The connector role mutation path that produced the event. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


