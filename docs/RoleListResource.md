# RoleListResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | Always `list`. (enum: list) | 
**data** | [**Vec<models::AssignedRoleDetails>**](AssignedRoleDetails.md) | Role assignments returned in the current page. | 
**has_more** | **bool** | Whether additional assignments are available when paginating. | 
**next** | Option<**String**> | Cursor to fetch the next page of results, or `null` when there are no more assignments. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


