# LiveSessionCreateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | [**models::ModelIdsLive**](ModelIdsLive.md) |  | 
**instructions** | Option<**String**> | Frontend instructions for voice, conversation, interruptions, and when to delegate. Start with the [Live prompting guide](https://developers.openai.com/api/docs/guides/live-prompting); put business rules and tool workflows in a separate [backend prompt](https://developers.openai.com/api/docs/guides/live-delegation#start-with-your-existing-backend-prompt). Limited to 16,384 client-supplied tokens. Omitted or blank instructions use server defaults. Immutable after startup. | [optional]
**input** | Option<[**Vec<models::LiveInitialItem>**](LiveInitialItem.md)> | Ordered text-only history supplied before startup. Supports developer, user, and assistant messages with one text part each; at most 128 messages and 8,192 rendered tokens in total. | [optional]
**audio** | Option<[**models::LiveInitialSessionAudioParam**](LiveInitialSessionAudioParam.md)> |  | [optional]
**delegation** | Option<[**models::LiveSessionCreateParamsDelegation**](LiveSessionCreateParams_delegation.md)> |  | [optional]
**store** | Option<**bool**> | Whether to store the session for later forking and recording download. Defaults to false for new sessions. | [optional]
**client** | Option<[**models::LiveClientConfigParam**](LiveClientConfigParam.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


