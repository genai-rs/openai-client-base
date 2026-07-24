# BetaMultiAgentCallOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**r#type** | **String** | The type of the multi-agent result. Always `multi_agent_call_output`. | 
**id** | **String** | The unique ID of the multi-agent call output item. | 
**call_id** | **String** | The unique ID of the multi-agent call. | 
**action** | [**models::BetaMultiAgentAction**](BetaMultiAgentAction.md) |  | 
**output** | [**Vec<models::BetaOutputTextContent>**](BetaOutputTextContent.md) | Text output returned by the multi-agent action. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


