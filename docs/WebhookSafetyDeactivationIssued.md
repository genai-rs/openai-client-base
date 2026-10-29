# WebhookSafetyDeactivationIssued

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique ID of the webhook event. | 
**object** | **Object** | Always `event`. (enum: event) | 
**created_at** | **i32** | The Unix timestamp in seconds when the event was created. | 
**r#type** | **Type** | Always `safety.deactivation_issued`. (enum: safety.deactivation_issued) | 
**data** | [**models::WebhookSafetyDeactivationIssuedData**](WebhookSafetyDeactivationIssuedData.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


