# SessionEventAgentSessionTurnCancelled

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `agent.session.turn.cancelled`. (enum: agent.session.turn.cancelled) | 
**event_id** | **String** | The unique ID of the event. | 
**session_id** | **String** | The ID of the session associated with the event. | 
**turn_id** | **String** | The ID of the turn associated with the event. | 
**turn** | [**models::TurnResource**](TurnResource.md) |  | 
**usage** | Option<[**models::TokenUsageResource**](TokenUsageResource.md)> | Token usage by the root agent during the turn, when available. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


