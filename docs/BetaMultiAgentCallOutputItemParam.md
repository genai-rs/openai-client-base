# BetaMultiAgentCallOutputItemParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTagParam**](Beta_AgentTagParam.md)> |  | [optional]
**id** | Option<**String**> | The unique ID of this multi-agent call output. | [optional]
**call_id** | **String** | The unique ID of the multi-agent call. | 
**r#type** | **String** | The item type. Always `multi_agent_call_output`. | 
**action** | [**models::BetaMultiAgentAction1**](BetaMultiAgentAction1.md) |  | 
**output** | [**Vec<models::BetaOutputTextContentParam>**](BetaOutputTextContentParam.md) | Text output returned by the multi-agent action. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


