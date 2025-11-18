# Role

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | Always `role`. | 
**id** | **String** | Identifier for the role. | 
**name** | **String** | Unique name for the role. | 
**description** | Option<**String**> |  | 
**permissions** | **Vec<String>** | Permissions granted by the role. | 
**resource_type** | **String** | Resource type the role is bound to (for example `api.organization` or `api.project`). | 
**predefined_role** | **bool** | Whether the role is predefined and managed by OpenAI. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


