# Message

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the message. Always set to `message`. | 
**id** | **String** | The unique ID of the message. | 
**status** | **String** | The status of item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API. | 
**role** | **String** | The role of the message. One of `unknown`, `user`, `assistant`, `system`, `critic`, `discriminator`, `developer`, or `tool`. | 
**content** | [**Vec<models::MessageContentInner>**](Message_content_inner.md) | The content of the message | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


