# VaultCredentialResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the credential. | 
**object** | **String** | The object type. Always `vault.credential`. | 
**vault_id** | **String** | The ID of the vault containing this credential. | 
**name** | **String** | The human-readable name of the credential. | 
**auth** | [**models::VaultCredentialAuthResource**](VaultCredentialAuthResource.md) |  | 
**metadata** | **std::collections::HashMap<String, String>** | Application-defined key-value pairs associated with this credential. | 
**created_at** | **i64** | The Unix timestamp, in seconds, when the credential was created. | 
**updated_at** | **i64** | The Unix timestamp, in seconds, when the credential was last updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


