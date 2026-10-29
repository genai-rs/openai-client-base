# Message

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the message. Always set to `message`. (enum: message) | 
**id** | **String** | The unique ID of the message. | 
**status** | [**models::MessageStatus**](MessageStatus.md) |  | 
**role** | [**models::MessageRole**](MessageRole.md) |  | 
**content** | [**Vec<models::MessageContentInner>**](MessageContentInner.md) | The content of the message | 
**phase** | Option<[**models::MessagePhase2**](MessagePhase2.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


