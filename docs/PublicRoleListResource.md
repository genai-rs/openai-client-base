# PublicRoleListResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | Always `list`. (enum: list) | 
**data** | [**Vec<models::Role>**](Role.md) | Roles returned in the current page. | 
**has_more** | **bool** | Whether more roles are available when paginating. | 
**next** | Option<**String**> | Cursor to fetch the next page of results, or `null` when there are no additional roles. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


