# OciExternalKeyParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | A friendly name to identify this config | 
**r#type** | **Type** | Defaults to `\"oci\"`. (enum: oci) | 
**region** | **String** | The OCI region containing the vault | 
**vault_ocid** | **String** | The customer OCI Vault OCID | 
**key_ocid** | **String** | The OCI Vault key OCID | 
**purpose** | Option<[**models::ExternalKeyPurpose**](ExternalKeyPurpose.md)> |  | [optional]
**key_version_ocid** | Option<**String**> | The exact OCI key version OCID for PSP decryption. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


