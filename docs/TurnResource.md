# TurnResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the turn. | 
**object** | [**models::TurnObjectResource**](TurnObjectResource.md) |  | 
**session_id** | **String** | The ID of the session that owns the turn. | 
**agent_id** | **String** | The ID of the agent that ran the turn. | 
**subagent_id** | Option<**String**> | The ID of the subagent that ran the turn, if applicable. | 
**status** | [**models::TurnStatusResource**](TurnStatusResource.md) |  | 
**created_at** | **i64** | The Unix timestamp, in seconds, used to order the turn by creation time. Subagent turns use their start time, falling back to completion time or the subagent opening time when the preceding timestamps are unavailable. | 
**started_at** | Option<**i64**> | The Unix timestamp, in seconds, when the turn started. | 
**completed_at** | Option<**i64**> | The Unix timestamp, in seconds, when the turn reached a terminal state. | 
**error** | Option<[**models::SessionTurnErrorResource**](SessionTurnErrorResource.md)> |  | 
**usage** | Option<[**models::TokenUsageResource**](TokenUsageResource.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


