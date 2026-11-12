# SessionEventAgentSessionTurnItemDone

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `agent.session.turn.item.done`.  Defaults to `\"agent.session.turn.item.done\"`. (enum: agent.session.turn.item.done) | 
**event_id** | **String** | The unique ID of the event. | 
**session_id** | **String** | The ID of the session associated with the event. | 
**turn_id** | Option<**String**> | The ID of the turn associated with the event, when applicable. | 
**output_index** | **i64** | The index of the output item in the turn output. | 
**item** | [**models::AgentOutputItemResource**](AgentOutputItemResource.md) |  | 
**obfuscation** | Option<**String**> | Random alphanumeric padding. Present when stream obfuscation is enabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


