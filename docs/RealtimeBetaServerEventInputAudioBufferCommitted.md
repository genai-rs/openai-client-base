# RealtimeBetaServerEventInputAudioBufferCommitted

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the server event. | 
**r#type** | Option<[**serde_json::Value**](.md)> |  | 
**previous_item_id** | Option<**String**> | The ID of the preceding item after which the new item will be inserted. Can be `null` if the item has no predecessor.  | [optional]
**item_id** | **String** | The ID of the user message item that will be created. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


