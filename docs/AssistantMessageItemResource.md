# AssistantMessageItemResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The item type. Always `message`. | 
**id** | **String** | The ID of the message. | 
**turn_id** | **String** | The ID of the turn that contains this item. | 
**role** | **String** | The role of the message author. Always `assistant`. | 
**status** | [**models::OutputItemStatusResource**](OutputItemStatusResource.md) |  | 
**content** | [**Vec<models::OutputTextResource>**](OutputTextResource.md) | The content of the message. | 
**phase** | Option<[**models::MessagePhaseResource**](MessagePhaseResource.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


