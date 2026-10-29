# SessionTurnTraceResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The root turn ID. Use this ID as the pagination anchor. | 
**object** | **Object** | The object type, which is always `agent.session.trace`. (enum: agent.session.trace) | 
**session_id** | **String** | The session that owns this trace. | 
**created_at** | **i64** | The Unix timestamp in seconds when the root turn was created. | 
**otlp** | **std::collections::HashMap<String, serde_json::Value>** | An OTLP JSON ExportTraceServiceRequest containing resourceSpans. Only currently published data is returned; later trace updates are not awaited. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


