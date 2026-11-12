# SessionEventAgentSessionInProgress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `agent.session.in_progress`.  Defaults to `\"agent.session.in_progress\"`. (enum: agent.session.in_progress) | 
**event_id** | **String** | The unique ID of the event. | 
**session** | [**models::SessionResource**](SessionResource.md) |  | 
**obfuscation** | Option<**String**> | Random alphanumeric padding. Present when stream obfuscation is enabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


