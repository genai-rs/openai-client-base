# LiveInstructionsAppendParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | Option<**String**> | Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id. | [optional]
**r#type** | **String** | The Live client event type. Always `session.instructions.append`. | 
**delegation_id** | Option<**String**> | Required, nullable. Set null for general session context, or use the ID from session.delegation.created for an existing client delegation. Non-null IDs are not accepted with Responses delegation. | 
**content** | **String** | Instruction text to append, limited to 500 tokens. This is a plain string, not an array of content parts. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


