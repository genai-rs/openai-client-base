# LiveSessionUpdateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delegation** | Option<[**models::LiveSessionUpdateParamsDelegation**](LiveSessionUpdateParamsDelegation.md)> |  | [optional]
**triggers** | Option<[**Vec<models::LiveDecisionTriggerInputParam2>**](LiveDecisionTriggerInputParam2.md)> | At most one Decisions trigger per transcript event type. Triggers run independently of delegation. On update, omit triggers to keep the current configuration, send a list to replace it, or send an empty list to disable triggers. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


