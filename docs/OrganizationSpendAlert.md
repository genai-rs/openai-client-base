# OrganizationSpendAlert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The identifier, which can be referenced in API endpoints. | 
**object** | **Object** | The object type, which is always `organization.spend_alert`. (enum: organization.spend_alert) | 
**threshold_amount** | **i32** | The alert threshold amount, in cents. | 
**currency** | **Currency** | The currency for the threshold amount. (enum: USD) | 
**interval** | **Interval** | The time interval for evaluating spend against the threshold. (enum: month) | 
**notification_channel** | [**models::SpendAlertNotificationChannel**](SpendAlertNotificationChannel.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


