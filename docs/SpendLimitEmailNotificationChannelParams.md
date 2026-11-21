# SpendLimitEmailNotificationChannelParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | Defaults to `\"email\"`. (enum: email) | 
**recipients** | Option<**Vec<String>**> | Additional email recipients. Omit to keep them unchanged; send [] to clear them. | [optional]
**subject_prefix** | Option<**String**> | Email subject prefix. Omit to keep it unchanged; send null or an empty string to clear it. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


