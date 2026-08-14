# BetaAgentMessageItemParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTagParam**](Beta_AgentTagParam.md)> |  | [optional]
**id** | Option<**String**> | The unique ID of this agent message item. | [optional]
**r#type** | **String** | The item type. Always `agent_message`. | 
**author** | **String** | The sending agent identity. | 
**recipient** | **String** | The destination agent identity. | 
**content** | [**Vec<models::BetaAgentMessageItemParamContentInner>**](BetaAgentMessageItemParam_content_inner.md) | Plaintext, image, or encrypted content sent between agents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


