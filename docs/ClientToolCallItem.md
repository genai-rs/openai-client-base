# ClientToolCallItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Identifier of the thread item. | 
**object** | **Object** | Type discriminator that is always `chatkit.thread_item`. (enum: chatkit.thread_item) | 
**created_at** | **i32** | Unix timestamp (in seconds) for when the item was created. | 
**thread_id** | **String** | Identifier of the parent thread. | 
**r#type** | **Type** | Type discriminator that is always `chatkit.client_tool_call`. (enum: chatkit.client_tool_call) | 
**status** | [**models::ClientToolCallStatus**](ClientToolCallStatus.md) |  | 
**call_id** | **String** | Identifier for the client tool call. | 
**name** | **String** | Tool name that was invoked. | 
**arguments** | **String** | JSON-encoded arguments that were sent to the tool. | 
**output** | Option<**String**> | JSON-encoded output captured from the tool. Defaults to null while execution is in progress. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


