# \HostedToolsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_project_hosted_tool_permissions**](HostedToolsApi.md#retrieve_project_hosted_tool_permissions) | **GET** /organization/projects/{project_id}/hosted_tool_permissions | Retrieve project hosted tool permissions
[**update_project_hosted_tool_permissions**](HostedToolsApi.md#update_project_hosted_tool_permissions) | **POST** /organization/projects/{project_id}/hosted_tool_permissions | Modify project hosted tool permissions



## retrieve_project_hosted_tool_permissions

> models::ProjectHostedToolPermissions retrieve_project_hosted_tool_permissions(project_id)
Retrieve project hosted tool permissions

Returns hosted tool permissions for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project. | [required] |

### Return type

[**models::ProjectHostedToolPermissions**](ProjectHostedToolPermissions.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_hosted_tool_permissions

> models::ProjectHostedToolPermissions update_project_hosted_tool_permissions(project_id, project_hosted_tool_permissions_update_request)
Modify project hosted tool permissions

Updates hosted tool permissions for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project. | [required] |
**project_hosted_tool_permissions_update_request** | [**ProjectHostedToolPermissionsUpdateRequest**](ProjectHostedToolPermissionsUpdateRequest.md) | The project hosted tool permissions update request payload. | [required] |

### Return type

[**models::ProjectHostedToolPermissions**](ProjectHostedToolPermissions.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

