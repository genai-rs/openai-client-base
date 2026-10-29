# BetaAgentMessageItemParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTagParam**](BetaAgentTagParam.md)> |  | [optional]
**id** | Option<**String**> | The unique ID of this agent message item. | [optional]
**r#type** | **Type** | The item type. Always `agent_message`. (enum: agent_message) | 
**author** | **String** | The sending agent identity. | 
**recipient** | **String** | The destination agent identity. | 
**content** | [**Vec<models::BetaAgentMessageItemParamContentInner>**](BetaAgentMessageItemParamContentInner.md) | Plaintext, image, or encrypted content sent between agents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


