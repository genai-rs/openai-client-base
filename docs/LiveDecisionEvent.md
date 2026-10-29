# LiveDecisionEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** |  (enum: decision.event) | 
**event_id** | **String** | The unique ID of this Live server event. | 
**delegation_id** | Option<**String**> | The Live delegation ID, or null for a transcript trigger. | 
**trigger_id** | Option<**String**> | The Live-assigned ID of this trigger firing, or null for a delegation. | 
**source_event_type** | Option<**SourceEventType**> |  (enum: session.input_transcript.delta, session.output_transcript.delta) | 
**source_event_id** | Option<**String**> | The event_id of the transcript delta that fired this trigger, or null for a delegation. | 
**event** | [**models::LiveDecisionCompleted**](LiveDecisionCompleted.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


