# SessionTurnListResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | The object type, which is always `list`. | 
**data** | [**Vec<models::TurnResource>**](TurnResource.md) | The resources returned in this page, in the requested sort order. | 
**first_id** | Option<**String**> | The ID of the first resource in `data`, or `null` if the page is empty. | 
**last_id** | Option<**String**> | The ID of the last resource in `data`, or `null` if the page is empty. Pass this as `after` with the same order and filters. | 
**has_more** | **bool** | Whether there are more resources to retrieve after this page. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


