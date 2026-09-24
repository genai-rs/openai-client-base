# CreateSubagentCallItemResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The item type. Always `create_subagent_call`. | 
**id** | **String** | The ID of the tool call item. | 
**turn_id** | **String** | The ID of the turn that contains this item. | 
**status** | [**models::FunctionCallStatusResource**](FunctionCallStatusResource.md) |  | 
**agent_id** | **String** | The ID of the agent that requested the subagent. | 
**content** | [**Vec<models::AgentContentResource>**](AgentContentResource.md) | The task given to the spawned agent. | 
**model** | Option<**String**> | The model requested for the spawned agent. | 
**reasoning_effort** | Option<**String**> | The reasoning effort requested for the spawned agent. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


