# AssistantMessageItemResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The item type. Always `message`. (enum: message) | 
**id** | **String** | The ID of the message. | 
**turn_id** | **String** | The ID of the turn that contains this item. | 
**role** | **Role** | The role of the message author. Always `assistant`. (enum: assistant) | 
**status** | [**models::OutputItemStatusResource**](OutputItemStatusResource.md) |  | 
**content** | [**Vec<models::OutputTextResource>**](OutputTextResource.md) | The content of the message. | 
**phase** | Option<[**models::MessagePhaseResource**](MessagePhaseResource.md)> | The phase of the assistant message. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


