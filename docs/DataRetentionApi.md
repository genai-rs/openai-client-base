# \DataRetentionApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_organization_data_retention**](DataRetentionApi.md#retrieve_organization_data_retention) | **GET** /organization/data_retention | Retrieves organization data retention controls.
[**retrieve_project_data_retention**](DataRetentionApi.md#retrieve_project_data_retention) | **GET** /organization/projects/{project_id}/data_retention | Retrieves project data retention controls.
[**update_organization_data_retention**](DataRetentionApi.md#update_organization_data_retention) | **POST** /organization/data_retention | Updates organization data retention controls.
[**update_project_data_retention**](DataRetentionApi.md#update_project_data_retention) | **POST** /organization/projects/{project_id}/data_retention | Updates project data retention controls.



## retrieve_organization_data_retention

> models::OrganizationDataRetention retrieve_organization_data_retention()
Retrieves organization data retention controls.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OrganizationDataRetention**](OrganizationDataRetention.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_project_data_retention

> models::ProjectDataRetention retrieve_project_data_retention(project_id)
Retrieves project data retention controls.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to inspect. | [required] |

### Return type

[**models::ProjectDataRetention**](ProjectDataRetention.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_data_retention

> models::OrganizationDataRetention update_organization_data_retention(update_organization_data_retention_body)
Updates organization data retention controls.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_organization_data_retention_body** | [**UpdateOrganizationDataRetentionBody**](UpdateOrganizationDataRetentionBody.md) | The desired organization data retention setting. | [required] |

### Return type

[**models::OrganizationDataRetention**](OrganizationDataRetention.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_data_retention

> models::ProjectDataRetention update_project_data_retention(project_id, update_project_data_retention_body)
Updates project data retention controls.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**update_project_data_retention_body** | [**UpdateProjectDataRetentionBody**](UpdateProjectDataRetentionBody.md) | The desired project data retention setting. | [required] |

### Return type

[**models::ProjectDataRetention**](ProjectDataRetention.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

