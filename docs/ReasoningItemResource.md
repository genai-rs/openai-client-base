# ReasoningItemResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The item type. Always `reasoning`. (enum: reasoning) | 
**id** | **String** | The ID of the reasoning item. | 
**turn_id** | **String** | The ID of the turn that contains this item. | 
**summary** | [**Vec<models::SummaryTextResource>**](SummaryTextResource.md) | The reasoning summaries produced by the agent. | 
**status** | Option<[**models::OutputItemStatusResource**](OutputItemStatusResource.md)> | The status of the reasoning item. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


