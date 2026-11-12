# AutomationInvocationResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The invocation ID. | 
**object** | **Object** | Always `agent.automation.invocation`.  Defaults to `\"agent.automation.invocation\"`. (enum: agent.automation.invocation) | 
**automation_id** | **String** | The automation that owns the invocation. | 
**created_at** | **i64** | The creation time as a Unix timestamp in seconds. | 
**source** | [**models::AutomationInvocationSourceResource**](AutomationInvocationSourceResource.md) |  | 
**scheduled_at** | Option<**i64**> | The intended occurrence as a Unix timestamp in seconds, or null for manual runs. | 
**session_id** | Option<**String**> | The target or newly created session ID, or null before one is available. | 
**status** | [**models::AutomationInvocationStatusResource**](AutomationInvocationStatusResource.md) |  | 
**skip_reason** | Option<**SkipReason**> | Why an occurrence was skipped, or null when it was not skipped.  Defaults to `\"pending_delivery\"`. (enum: pending_delivery, ) | 
**error** | Option<[**models::AutomationInvocationErrorResource**](AutomationInvocationErrorResource.md)> | The permanent delivery failure, or null when no failure occurred. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


