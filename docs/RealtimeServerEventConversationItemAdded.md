# RealtimeServerEventConversationItemAdded

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the server event. | 
**r#type** | **String** | The event type, must be `conversation.item.added`. | 
**previous_item_id** | Option<**String**> | The ID of the item that precedes this one, if any. This is used to maintain ordering when items are inserted.  | [optional]
**item** | [**models::RealtimeConversationItem**](RealtimeConversationItem.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


