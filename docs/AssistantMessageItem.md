# AssistantMessageItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Identifier of the thread item. | 
**object** | **String** | Type discriminator that is always `chatkit.thread_item`. | 
**created_at** | **i32** | Unix timestamp (in seconds) for when the item was created. | 
**thread_id** | **String** | Identifier of the parent thread. | 
**r#type** | **String** | Type discriminator that is always `chatkit.assistant_message`. | 
**content** | [**Vec<models::ResponseOutputText>**](ResponseOutputText.md) | Ordered assistant response segments. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


