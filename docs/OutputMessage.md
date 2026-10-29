# OutputMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique ID of the output message.  | 
**r#type** | **Type** | The type of the output message. Always `message`.  (enum: message) | 
**role** | **Role** | The role of the output message. Always `assistant`.  (enum: assistant) | 
**content** | [**Vec<models::OutputMessageContent>**](OutputMessageContent.md) | The content of the output message.  | 
**phase** | Option<[**models::MessagePhase**](MessagePhase.md)> |  | [optional]
**status** | **Status** | The status of the message input. One of `in_progress`, `completed`, or `incomplete`. Populated when input items are returned via API.  (enum: in_progress, completed, incomplete) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


