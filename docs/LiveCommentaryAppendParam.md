# LiveCommentaryAppendParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | Option<**String**> | Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id. | [optional]
**r#type** | **String** | The Live client event type. Always `session.commentary.append`. | 
**delegation_id** | Option<**String**> | Required, nullable. Set null for general session context, or use the ID from session.delegation.created for an existing client delegation. Non-null IDs are not accepted with Responses delegation. | 
**content** | **String** | Speakable context for the Live model, limited to 500 tokens. Use this for a result the model should communicate; use session.thinking.append for silent context. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


