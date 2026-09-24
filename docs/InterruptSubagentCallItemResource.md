# InterruptSubagentCallItemResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The item type. Always `interrupt_subagent_call`. | 
**id** | **String** | The ID of the tool call item. | 
**turn_id** | **String** | The ID of the turn that contains this item. | 
**status** | [**models::FunctionCallStatusResource**](FunctionCallStatusResource.md) |  | 
**sender_agent_id** | **String** | The ID of the agent requesting the interrupt. | 
**recipient_agent_id** | **String** | The ID of the agent to interrupt. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


