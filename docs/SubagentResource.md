# SubagentResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the subagent. | 
**object** | [**models::SubagentObjectResource**](SubagentObjectResource.md) |  | 
**session_id** | **String** | The ID of the session that owns the subagent. | 
**name** | Option<**String**> | The runner-assigned nickname, or null when unavailable. | 
**instructions** | Option<[**Vec<models::AgentContentResource>**](AgentContentResource.md)> | Initial task content, or null when unavailable. Text may contain placeholders for images or audio when only a preview is available. | 
**parent_agent_id** | **String** | The ID of the agent that created this subagent. | 
**status** | [**models::SubagentStatusResource**](SubagentStatusResource.md) |  | 
**opened_at** | **i64** | The Unix timestamp, in seconds, when the subagent was first opened. Resuming does not change it. | 
**closed_at** | Option<**i64**> | The Unix timestamp, in seconds, when the subagent was closed. Null while active, including after resume. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


