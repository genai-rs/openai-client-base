# AgentMessageItemResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the message. | 
**turn_id** | **String** | The ID of the turn that contains this item. | 
**r#type** | **String** | The item type. Always `agent_message`. | 
**sender_agent_id** | **String** | The ID or name of the sending agent. | 
**recipient_agent_id** | **String** | The ID or name of the receiving agent. | 
**content** | [**Vec<models::AgentContentResource>**](AgentContentResource.md) | The content exchanged between the agents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


