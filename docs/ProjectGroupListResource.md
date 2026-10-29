# ProjectGroupListResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | Always `list`. (enum: list) | 
**data** | [**Vec<models::ProjectGroup>**](ProjectGroup.md) | Project group memberships returned in the current page. | 
**has_more** | **bool** | Whether additional project group memberships are available. | 
**next** | Option<**String**> | Cursor to fetch the next page of results, or `null` when there are no more results. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


