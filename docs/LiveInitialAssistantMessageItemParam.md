# LiveInitialAssistantMessageItemParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | An optional identifier for the supplied history message. Live uses the message’s role and text to initialize the conversation. | [optional]
**r#type** | Option<**String**> | The history item type. Always `message`. | [optional]
**status** | Option<[**models::LiveInitialMessageStatus**](LiveInitialMessageStatus.md)> |  | [optional]
**role** | **String** | The author of this history message. Always `assistant`. | 
**content** | [**Vec<models::LiveInitialAssistantMessageItemParamContentInner>**](LiveInitialAssistantMessageItemParam_content_inner.md) | The message content. Supply exactly one text part for the initial Live conversation history. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


