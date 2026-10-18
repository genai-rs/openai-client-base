# LiveCallAcceptSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | [**models::ModelIdsLive**](ModelIdsLive.md) |  | 
**instructions** | Option<**String**> | Frontend instructions for voice, conversation, interruptions, and when to delegate. Start with the [Live prompting guide](https://developers.openai.com/api/docs/guides/live-prompting); put business rules and tool workflows in a separate [backend prompt](https://developers.openai.com/api/docs/guides/live-delegation#start-with-your-existing-backend-prompt). Limited to 16,384 client-supplied tokens. Omitted or blank instructions use server defaults. Immutable after startup. | [optional]
**audio** | Option<[**models::LiveMediaSessionAudioParam**](LiveMediaSessionAudioParam.md)> |  | [optional]
**delegation** | Option<[**models::Delegation**](delegation.md)> |  | [optional]
**triggers** | Option<[**Vec<models::LiveDecisionTriggerInputParam>**](LiveDecisionTriggerInputParam.md)> | At most one Decisions trigger per transcript event type. Triggers run independently of delegation. On update, omit triggers to keep the current configuration, send a list to replace it, or send an empty list to disable triggers. | [optional]
**store** | Option<**bool**> | Whether to store the session for later forking and recording download. Defaults to false for new sessions. | [optional]
**input** | Option<[**Vec<models::LiveInitialItem>**](LiveInitialItem.md)> | Ordered text-only history supplied before startup. Supports developer, user, and assistant messages with one text part each; at most 128 messages and a configured rendered-token limit (8,192 by default). | [optional]
**r#type** | **String** | The session type. Always `live`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


