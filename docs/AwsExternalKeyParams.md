# AwsExternalKeyParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | A friendly name to identify this config | 
**r#type** | **Type** | Defaults to `\"aws\"`. (enum: aws) | 
**role_arn** | **String** | The Role ARN that OpenAI will assume to call your KMS | 
**kms_arn** | **String** | The Key Management System ARN for the master key you manage | 
**external_id** | **String** | The ExternalID that OpenAI will pass in the AssumeRole call for the customer's AWS account. This must be the OpenAI organization ID or an API project ID that belongs to the organization. | 
**purpose** | Option<[**models::ExternalKeyPurpose**](ExternalKeyPurpose.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


