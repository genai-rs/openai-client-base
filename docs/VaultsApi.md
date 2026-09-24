# \VaultsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_vault**](VaultsApi.md#create_vault) | **POST** /vaults | Create a vault
[**create_vault_credential**](VaultsApi.md#create_vault_credential) | **POST** /vaults/{vault_id}/credentials | Create a vault credential
[**delete_vault**](VaultsApi.md#delete_vault) | **DELETE** /vaults/{vault_id} | Delete a vault
[**delete_vault_credential**](VaultsApi.md#delete_vault_credential) | **DELETE** /vaults/{vault_id}/credentials/{credential_id} | Delete a vault credential
[**list_vault_credentials**](VaultsApi.md#list_vault_credentials) | **GET** /vaults/{vault_id}/credentials | List vault credentials
[**list_vaults**](VaultsApi.md#list_vaults) | **GET** /vaults | List vaults
[**retrieve_vault**](VaultsApi.md#retrieve_vault) | **GET** /vaults/{vault_id} | Retrieve a vault
[**retrieve_vault_credential**](VaultsApi.md#retrieve_vault_credential) | **GET** /vaults/{vault_id}/credentials/{credential_id} | Retrieve a vault credential
[**rotate_vault_credential**](VaultsApi.md#rotate_vault_credential) | **POST** /vaults/{vault_id}/credentials/{credential_id} | Update a vault credential



## create_vault

> models::VaultResource create_vault(create_vault_params)
Create a vault

Creates a vault for the current project. See [vaults](https://developers.openai.com/api/docs/guides/agents-api/tools/vaults).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_vault_params** | Option<[**CreateVaultParams**](CreateVaultParams.md)> |  |  |

### Return type

[**models::VaultResource**](VaultResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vault_credential

> models::VaultCredentialResource create_vault_credential(vault_id, create_vault_credential_params)
Create a vault credential

Creates a vault credential. Secret values are write-only and are never returned. See [vaults](https://developers.openai.com/api/docs/guides/agents-api/tools/vaults).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_id** | **String** | The ID of the vault. | [required] |
**create_vault_credential_params** | Option<[**CreateVaultCredentialParams**](CreateVaultCredentialParams.md)> |  |  |

### Return type

[**models::VaultCredentialResource**](VaultCredentialResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vault

> models::DeletedVaultResource delete_vault(vault_id)
Delete a vault

Deletes a vault and all its credentials. See [vaults](https://developers.openai.com/api/docs/guides/agents-api/tools/vaults).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_id** | **String** | The ID of the vault. | [required] |

### Return type

[**models::DeletedVaultResource**](DeletedVaultResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vault_credential

> models::DeletedVaultCredentialResource delete_vault_credential(vault_id, credential_id)
Delete a vault credential

Deletes a vault credential. See [vaults](https://developers.openai.com/api/docs/guides/agents-api/tools/vaults).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_id** | **String** | The ID of the vault. | [required] |
**credential_id** | **String** | The ID of the vault credential. | [required] |

### Return type

[**models::DeletedVaultCredentialResource**](DeletedVaultCredentialResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_vault_credentials

> models::VaultCredentialListResource list_vault_credentials(vault_id, order, limit, status, after)
List vault credentials

Lists a vault's credentials using ID-based pagination without returning secret values. See [vaults](https://developers.openai.com/api/docs/guides/agents-api/tools/vaults).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_id** | **String** | The ID of the vault. | [required] |
**order** | Option<[**ListOrderParam**](.md)> | Sort order by the `created_at` timestamp. Use `asc` for ascending order or `desc` for descending order. Defaults to `desc`. |  |
**limit** | Option<**i64**> | The maximum number of resources to return. Defaults to 20. Values are clamped between 1 and 100. |  |
**status** | Option<**String**> | Filter by one status or a list, such as `status=active` or `status[]=active&status[]=archived`. Both statuses are included by default. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |

### Return type

[**models::VaultCredentialListResource**](VaultCredentialListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_vaults

> models::VaultListResource list_vaults(order, limit, status, after)
List vaults

Lists vaults using ID-based pagination. See [vaults](https://developers.openai.com/api/docs/guides/agents-api/tools/vaults).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order** | Option<[**ListOrderParam**](.md)> | Sort order by the `created_at` timestamp. Use `asc` for ascending order or `desc` for descending order. Defaults to `desc`. |  |
**limit** | Option<**i64**> | The maximum number of resources to return. Defaults to 20. Values are clamped between 1 and 100. |  |
**status** | Option<[**VaultStatusFilterParam**](.md)> | Filter by one status or a list, such as `status=active` or `status[]=active&status[]=archived`. Both statuses are included by default. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |

### Return type

[**models::VaultListResource**](VaultListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_vault

> models::VaultResource retrieve_vault(vault_id)
Retrieve a vault

Retrieves a vault by its ID. See [vaults](https://developers.openai.com/api/docs/guides/agents-api/tools/vaults).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_id** | **String** | The ID of the vault. | [required] |

### Return type

[**models::VaultResource**](VaultResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_vault_credential

> models::VaultCredentialResource retrieve_vault_credential(vault_id, credential_id)
Retrieve a vault credential

Retrieves vault credential metadata without returning secret values. See [vaults](https://developers.openai.com/api/docs/guides/agents-api/tools/vaults).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_id** | **String** | The ID of the vault. | [required] |
**credential_id** | **String** | The ID of the vault credential. | [required] |

### Return type

[**models::VaultCredentialResource**](VaultCredentialResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_vault_credential

> models::VaultCredentialResource rotate_vault_credential(vault_id, credential_id, rotate_vault_credential_params)
Update a vault credential

Updates credential metadata or rotates its write-only secret. See [vaults](https://developers.openai.com/api/docs/guides/agents-api/tools/vaults).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_id** | **String** | The ID of the vault. | [required] |
**credential_id** | **String** | The ID of the vault credential. | [required] |
**rotate_vault_credential_params** | Option<[**RotateVaultCredentialParams**](RotateVaultCredentialParams.md)> |  |  |

### Return type

[**models::VaultCredentialResource**](VaultCredentialResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

