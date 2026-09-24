# LiveResponseEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the Live server event. | 
**client_event_id** | Option<**String**> | The event_id of the client command associated with this server event, when supplied. | [optional]
**r#type** | **String** | The event type, always `response.event`. | 
**delegation_id** | Option<**String**> | The Live delegation associated with the nested Responses event. May be null or omitted when the event cannot be correlated with a delegation. | [optional]
**event** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | The nested Responses streaming event. Dispatch on its type field. Response lifecycle snapshots omit input and clear instructions, tools, and output to keep messages small; consume granular output events for the generated content. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


