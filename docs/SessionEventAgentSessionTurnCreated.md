# SessionEventAgentSessionTurnCreated

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `agent.session.turn.created`.  Defaults to `\"agent.session.turn.created\"`. (enum: agent.session.turn.created) | 
**event_id** | **String** | The unique ID of the event. | 
**session_id** | **String** | The ID of the session associated with the event. | 
**turn_id** | **String** | The ID of the turn associated with the event. | 
**turn** | [**models::TurnResource**](TurnResource.md) |  | 
**obfuscation** | Option<**String**> | Random alphanumeric padding. Present when stream obfuscation is enabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


