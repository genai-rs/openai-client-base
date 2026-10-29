# AssignedRoleDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Identifier for the role. | 
**name** | **String** | Name of the role. | 
**permissions** | **Vec<String>** | Permissions associated with the role. | 
**resource_type** | **String** | Resource type the role applies to. | 
**predefined_role** | **bool** | Whether the role is predefined by OpenAI. | 
**description** | Option<**String**> | Description of the role. | 
**created_at** | Option<**i32**> | When the role was created. | 
**updated_at** | Option<**i32**> | When the role was last updated. | 
**created_by** | Option<**String**> | Identifier of the actor who created the role. | 
**created_by_user_obj** | Option<**std::collections::HashMap<String, serde_json::Value>**> | User details for the actor that created the role, when available. | 
**metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Arbitrary metadata stored on the role. | 
**assignment_sources** | Option<[**Vec<models::AssignedRoleDetailsAssignmentSourcesInner>**](AssignedRoleDetailsAssignmentSourcesInner.md)> | Principals from which the role assignment is inherited, when available. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


