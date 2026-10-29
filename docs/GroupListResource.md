# GroupListResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | Always `list`. (enum: list) | 
**data** | [**Vec<models::GroupResponse>**](GroupResponse.md) | Groups returned in the current page. | 
**has_more** | **bool** | Whether additional groups are available when paginating. | 
**next** | Option<**String**> | Cursor to fetch the next page of results, or `null` if there are no more results. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


