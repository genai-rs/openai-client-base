# LiveThinkingAppendParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | Option<**String**> | Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id. | [optional]
**r#type** | **String** | The Live client event type. Always `session.thinking.append`. | 
**delegation_id** | Option<**String**> | Required, nullable. Set null for general session context, or use the ID from session.delegation.created for an existing client delegation. Non-null IDs are not accepted with Responses delegation. | 
**content** | **String** | Silent reasoning or progress context, limited to 500 tokens. It does not directly request speech, but can influence later speech and is not a secrecy boundary. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


