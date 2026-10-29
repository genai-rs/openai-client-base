# BetaAgentMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**r#type** | **Type** | The type of the item. Always `agent_message`. (enum: agent_message) | 
**id** | **String** | The unique ID of the agent message. | 
**author** | **String** | The sending agent identity. | 
**recipient** | **String** | The destination agent identity. | 
**content** | [**Vec<models::BetaAgentMessageContentInner>**](BetaAgentMessageContentInner.md) | Encrypted content sent between agents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


