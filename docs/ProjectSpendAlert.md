# ProjectSpendAlert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The identifier, which can be referenced in API endpoints. | 
**object** | **String** | The object type, which is always `project.spend_alert`. | 
**threshold_amount** | **i32** | The alert threshold amount, in cents. | 
**currency** | **String** | The currency for the threshold amount. | 
**interval** | **String** | The time interval for evaluating spend against the threshold. | 
**notification_channel** | [**models::SpendAlertNotificationChannel**](SpendAlertNotificationChannel.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


