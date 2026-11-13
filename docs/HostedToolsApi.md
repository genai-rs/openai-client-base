# \HostedToolsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_project_hosted_tool_permissions**](HostedToolsApi.md#retrieve_project_hosted_tool_permissions) | **GET** /organization/projects/{project_id}/hosted_tool_permissions | Retrieve project hosted tool permissions
[**update_project_hosted_tool_permissions**](HostedToolsApi.md#update_project_hosted_tool_permissions) | **POST** /organization/projects/{project_id}/hosted_tool_permissions | Modify project hosted tool permissions



## retrieve_project_hosted_tool_permissions

> models::ProjectHostedToolPermissions retrieve_project_hosted_tool_permissions(project_id, options)
Retrieve project hosted tool permissions

Returns hosted tool permissions for a project.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_project_hosted_tool_permissions().configuration(&configuration).project_id(..).options(&options).call().await
```

### Return type

[**models::ProjectHostedToolPermissions**](ProjectHostedToolPermissions.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_hosted_tool_permissions

> models::ProjectHostedToolPermissions update_project_hosted_tool_permissions(project_id, project_hosted_tool_permissions_update_request, options)
Modify project hosted tool permissions

Updates hosted tool permissions for a project.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project. | [required] |
**project_hosted_tool_permissions_update_request** | [**ProjectHostedToolPermissionsUpdateRequest**](ProjectHostedToolPermissionsUpdateRequest.md) | The project hosted tool permissions update request payload. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
update_project_hosted_tool_permissions().configuration(&configuration).project_id(..).project_hosted_tool_permissions_update_request(..).options(&options).call().await
```

### Return type

[**models::ProjectHostedToolPermissions**](ProjectHostedToolPermissions.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

