# TaskGroupItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Identifier of the thread item. | 
**object** | **Object** | Type discriminator that is always `chatkit.thread_item`. (enum: chatkit.thread_item) | 
**created_at** | **i32** | Unix timestamp (in seconds) for when the item was created. | 
**thread_id** | **String** | Identifier of the parent thread. | 
**r#type** | **Type** | Type discriminator that is always `chatkit.task_group`. (enum: chatkit.task_group) | 
**tasks** | [**Vec<models::TaskGroupTask>**](TaskGroupTask.md) | Tasks included in the group. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


