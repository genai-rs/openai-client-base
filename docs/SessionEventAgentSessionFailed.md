# SessionEventAgentSessionFailed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `agent.session.failed`.  Defaults to `\"agent.session.failed\"`. (enum: agent.session.failed) | 
**event_id** | **String** | The unique ID of the event. | 
**session** | [**models::SessionResource**](SessionResource.md) |  | 
**obfuscation** | Option<**String**> | Random alphanumeric padding. Present when stream obfuscation is enabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


