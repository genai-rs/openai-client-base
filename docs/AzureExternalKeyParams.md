# AzureExternalKeyParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | A friendly name to identify this config | 
**r#type** | **Type** | Defaults to `\"azure\"`. (enum: azure) | 
**tenant_id** | **String** | Your Azure tenant UUID | 
**vault_uri** | **String** | The URI of the Azure vault containing the master key you manage | 
**key_name** | **String** | The name of the Azure Key Vault master key that you manage | 
**purpose** | Option<[**models::ExternalKeyPurpose**](ExternalKeyPurpose.md)> |  | [optional]
**key_version** | Option<**String**> | An exact Azure Key Vault key version. Required for PSP decryption; not accepted for EKM. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


