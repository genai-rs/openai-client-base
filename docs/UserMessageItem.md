# UserMessageItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Identifier of the thread item. | 
**object** | **Object** | Type discriminator that is always `chatkit.thread_item`. (enum: chatkit.thread_item) | 
**created_at** | **i32** | Unix timestamp (in seconds) for when the item was created. | 
**thread_id** | **String** | Identifier of the parent thread. | 
**r#type** | **Type** |  (enum: chatkit.user_message) | 
**content** | [**Vec<models::UserMessageItemContentInner>**](UserMessageItemContentInner.md) | Ordered content elements supplied by the user. | 
**attachments** | [**Vec<models::Attachment>**](Attachment.md) | Attachments associated with the user message. Defaults to an empty list. | 
**inference_options** | Option<[**models::InferenceOptions**](InferenceOptions.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


