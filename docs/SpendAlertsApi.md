# \SpendAlertsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_spend_alert**](SpendAlertsApi.md#create_organization_spend_alert) | **POST** /organization/spend_alerts | Creates an organization spend alert.
[**create_project_spend_alert**](SpendAlertsApi.md#create_project_spend_alert) | **POST** /organization/projects/{project_id}/spend_alerts | Creates a project spend alert.
[**delete_organization_spend_alert**](SpendAlertsApi.md#delete_organization_spend_alert) | **DELETE** /organization/spend_alerts/{alert_id} | Deletes an organization spend alert.
[**delete_project_spend_alert**](SpendAlertsApi.md#delete_project_spend_alert) | **DELETE** /organization/projects/{project_id}/spend_alerts/{alert_id} | Deletes a project spend alert.
[**list_organization_spend_alerts**](SpendAlertsApi.md#list_organization_spend_alerts) | **GET** /organization/spend_alerts | Lists organization spend alerts.
[**list_project_spend_alerts**](SpendAlertsApi.md#list_project_spend_alerts) | **GET** /organization/projects/{project_id}/spend_alerts | Lists project spend alerts.
[**retrieve_organization_spend_alert**](SpendAlertsApi.md#retrieve_organization_spend_alert) | **GET** /organization/spend_alerts/{alert_id} | Retrieves an organization spend alert.
[**retrieve_project_spend_alert**](SpendAlertsApi.md#retrieve_project_spend_alert) | **GET** /organization/projects/{project_id}/spend_alerts/{alert_id} | Retrieves a project spend alert.
[**update_organization_spend_alert**](SpendAlertsApi.md#update_organization_spend_alert) | **POST** /organization/spend_alerts/{alert_id} | Updates an organization spend alert.
[**update_project_spend_alert**](SpendAlertsApi.md#update_project_spend_alert) | **POST** /organization/projects/{project_id}/spend_alerts/{alert_id} | Updates a project spend alert.



## create_organization_spend_alert

> models::OrganizationSpendAlert create_organization_spend_alert(create_spend_alert_body)
Creates an organization spend alert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_spend_alert_body** | [**CreateSpendAlertBody**](CreateSpendAlertBody.md) | Parameters for the organization spend alert you want to create. | [required] |

### Return type

[**models::OrganizationSpendAlert**](OrganizationSpendAlert.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_spend_alert

> models::ProjectSpendAlert create_project_spend_alert(project_id, create_spend_alert_body)
Creates a project spend alert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**create_spend_alert_body** | [**CreateSpendAlertBody**](CreateSpendAlertBody.md) | Parameters for the project spend alert you want to create. | [required] |

### Return type

[**models::ProjectSpendAlert**](ProjectSpendAlert.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_spend_alert

> models::OrganizationSpendAlertDeletedResource delete_organization_spend_alert(alert_id)
Deletes an organization spend alert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the spend alert to delete. | [required] |

### Return type

[**models::OrganizationSpendAlertDeletedResource**](OrganizationSpendAlertDeletedResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_spend_alert

> models::ProjectSpendAlertDeletedResource delete_project_spend_alert(project_id, alert_id)
Deletes a project spend alert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**alert_id** | **String** | The ID of the spend alert to delete. | [required] |

### Return type

[**models::ProjectSpendAlertDeletedResource**](ProjectSpendAlertDeletedResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_spend_alerts

> models::OrganizationSpendAlertListResource list_organization_spend_alerts(limit, order, after, before)
Lists organization spend alerts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | A limit on the number of spend alerts to return. Defaults to 20. |  |
**order** | Option<**String**> | Sort order for the returned spend alerts. |  |[default to asc]
**after** | Option<**String**> | Cursor for pagination. Provide the ID of the last spend alert from the previous response to fetch the next page. |  |
**before** | Option<**String**> | Cursor for pagination. Provide the ID of the first spend alert from the previous response to fetch the previous page. |  |

### Return type

[**models::OrganizationSpendAlertListResource**](OrganizationSpendAlertListResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_spend_alerts

> models::ProjectSpendAlertListResource list_project_spend_alerts(project_id, limit, order, after, before)
Lists project spend alerts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to inspect. | [required] |
**limit** | Option<**i32**> | A limit on the number of spend alerts to return. Defaults to 20. |  |
**order** | Option<**String**> | Sort order for the returned spend alerts. |  |[default to asc]
**after** | Option<**String**> | Cursor for pagination. Provide the ID of the last spend alert from the previous response to fetch the next page. |  |
**before** | Option<**String**> | Cursor for pagination. Provide the ID of the first spend alert from the previous response to fetch the previous page. |  |

### Return type

[**models::ProjectSpendAlertListResource**](ProjectSpendAlertListResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_organization_spend_alert

> models::OrganizationSpendAlert retrieve_organization_spend_alert(alert_id)
Retrieves an organization spend alert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the spend alert to retrieve. | [required] |

### Return type

[**models::OrganizationSpendAlert**](OrganizationSpendAlert.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_project_spend_alert

> models::ProjectSpendAlert retrieve_project_spend_alert(project_id, alert_id)
Retrieves a project spend alert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project. | [required] |
**alert_id** | **String** | The ID of the spend alert to retrieve. | [required] |

### Return type

[**models::ProjectSpendAlert**](ProjectSpendAlert.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_spend_alert

> models::OrganizationSpendAlert update_organization_spend_alert(alert_id, create_spend_alert_body)
Updates an organization spend alert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the spend alert to update. | [required] |
**create_spend_alert_body** | [**CreateSpendAlertBody**](CreateSpendAlertBody.md) | Fields to update on the organization spend alert. | [required] |

### Return type

[**models::OrganizationSpendAlert**](OrganizationSpendAlert.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_spend_alert

> models::ProjectSpendAlert update_project_spend_alert(project_id, alert_id, create_spend_alert_body)
Updates a project spend alert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**alert_id** | **String** | The ID of the spend alert to update. | [required] |
**create_spend_alert_body** | [**CreateSpendAlertBody**](CreateSpendAlertBody.md) | Fields to update on the project spend alert. | [required] |

### Return type

[**models::ProjectSpendAlert**](ProjectSpendAlert.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

