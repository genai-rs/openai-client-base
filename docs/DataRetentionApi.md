# \DataRetentionApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_organization_data_retention**](DataRetentionApi.md#retrieve_organization_data_retention) | **GET** /organization/data_retention | Retrieve organization data retention
[**retrieve_project_data_retention**](DataRetentionApi.md#retrieve_project_data_retention) | **GET** /organization/projects/{project_id}/data_retention | Retrieve project data retention
[**update_organization_data_retention**](DataRetentionApi.md#update_organization_data_retention) | **POST** /organization/data_retention | Update organization data retention
[**update_project_data_retention**](DataRetentionApi.md#update_project_data_retention) | **POST** /organization/projects/{project_id}/data_retention | Update project data retention



## retrieve_organization_data_retention

> models::OrganizationDataRetention retrieve_organization_data_retention(options)
Retrieve organization data retention

Retrieves organization data retention controls.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_organization_data_retention().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::OrganizationDataRetention**](OrganizationDataRetention.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_project_data_retention

> models::ProjectDataRetention retrieve_project_data_retention(project_id, options)
Retrieve project data retention

Retrieves project data retention controls.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to inspect. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_project_data_retention().configuration(&configuration).project_id(..).options(&options).call().await
```

### Return type

[**models::ProjectDataRetention**](ProjectDataRetention.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_data_retention

> models::OrganizationDataRetention update_organization_data_retention(update_organization_data_retention_body, options)
Update organization data retention

Updates organization data retention controls.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_organization_data_retention_body** | [**UpdateOrganizationDataRetentionBody**](UpdateOrganizationDataRetentionBody.md) | The desired organization data retention setting. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
update_organization_data_retention().configuration(&configuration).update_organization_data_retention_body(..).options(&options).call().await
```

### Return type

[**models::OrganizationDataRetention**](OrganizationDataRetention.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_data_retention

> models::ProjectDataRetention update_project_data_retention(project_id, update_project_data_retention_body, options)
Update project data retention

Updates project data retention controls.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**update_project_data_retention_body** | [**UpdateProjectDataRetentionBody**](UpdateProjectDataRetentionBody.md) | The desired project data retention setting. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
update_project_data_retention().configuration(&configuration).project_id(..).update_project_data_retention_body(..).options(&options).call().await
```

### Return type

[**models::ProjectDataRetention**](ProjectDataRetention.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

