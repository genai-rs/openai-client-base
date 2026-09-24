# SessionEventAgentSessionTurnOutputTextDone

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `agent.session.turn.output_text.done`. | 
**event_id** | **String** | The unique ID of the event. | 
**session_id** | **String** | The ID of the session associated with the event. | 
**turn_id** | Option<**String**> | The ID of the turn associated with the event, when applicable. | 
**item_id** | **String** | The ID of the message item. | 
**output_index** | **i64** | The index of the item in the turn output. | 
**content_index** | **i64** | The index of the content part in the message. | 
**text** | **String** | The complete output text. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


