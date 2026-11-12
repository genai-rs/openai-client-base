# GcpExternalKeyParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | A friendly name to identify this config | 
**r#type** | **Type** | Defaults to `\"gcp\"`. (enum: gcp) | 
**workload_identity_project_number** | **String** | The numeric GCP project number where you registered OpenAI's workload identity | 
**workload_identity_pool_id** | **String** | The pool containing the Workload Identity provider that you registered for OpenAI | 
**workload_identity_provider_id** | **String** | The Workload Identity provider that you registered for OpenAI | 
**kms_project_id** | **String** | The string GCP project name where your KMS resides | 
**kms_key_ring_name** | **String** | The Key Management System key ring for the master key you manage | 
**kms_key_name** | **String** | The name of the Key Management System master key you manage | 
**kms_key_location** | **String** | The region where your Key Management System master key is located | 
**audience** | **String** | The aud field in the identity token OpenAI will pass in the call to the customer's GCP STS. This must be the OpenAI organization ID or an API project ID that belongs to the organization. | 
**purpose** | Option<[**models::ExternalKeyPurpose**](ExternalKeyPurpose.md)> |  | [optional]
**kms_key_version** | Option<**String**> | An exact CryptoKeyVersion number. Required for PSP decryption; not accepted for EKM. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


