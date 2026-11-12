# SessionEventAgentSessionEnvironmentDisconnected

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `agent.session.environment.disconnected`.  Defaults to `\"agent.session.environment.disconnected\"`. (enum: agent.session.environment.disconnected) | 
**event_id** | **String** | The unique ID of the event. | 
**session_id** | **String** | The ID of the session associated with the event. | 
**turn_id** | Option<**String**> | The ID of the turn associated with the event, when applicable. | 
**environment** | [**models::SessionEnvironmentStateResource**](SessionEnvironmentStateResource.md) |  | 
**obfuscation** | Option<**String**> | Random alphanumeric padding. Present when stream obfuscation is enabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


