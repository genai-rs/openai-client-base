# MessageItemResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The item type. Always `message`. | 
**id** | Option<**String**> | The ID of this item, or null for legacy user messages whose ID was not recorded. | 
**turn_id** | **String** | The ID of the turn that contains this item. | 
**role** | [**models::SessionMessageRoleResource**](SessionMessageRoleResource.md) |  | 
**content** | [**Vec<models::MessageContentResource>**](MessageContentResource.md) | The content of the message. User messages contain input text or images; assistant messages contain output text. | 
**status** | [**models::OutputItemStatusResource**](OutputItemStatusResource.md) |  | 
**phase** | Option<[**models::MessagePhaseResource**](MessagePhaseResource.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


