# SessionEventAgentSessionEnvironmentReset

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `agent.session.environment.reset`. | 
**event_id** | **String** | The unique ID of the event. | 
**session_id** | **String** | The ID of the session associated with the event. | 
**turn_id** | Option<**String**> | The associated turn, when applicable. | 
**environment_id** | **String** | The stable environment ID, retained across sandbox replacements. | 
**reset_count** | **i64** | Monotonically increasing reset number. Repeated notifications share this number. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


