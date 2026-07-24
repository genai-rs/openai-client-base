# BetaMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**r#type** | **String** | The type of the message. Always set to `message`. | 
**id** | **String** | The unique ID of the message. | 
**status** | [**models::BetaMessageStatus**](BetaMessageStatus.md) |  | 
**role** | [**models::BetaMessageRole**](BetaMessageRole.md) |  | 
**content** | [**Vec<models::BetaAgentMessageContentInner>**](BetaAgentMessage_content_inner.md) | The content of the message | 
**phase** | Option<[**models::BetaMessagePhase2**](BetaMessagePhase2.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


