# \ProjectGroupRoleAssignmentsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_project_group_role**](ProjectGroupRoleAssignmentsApi.md#assign_project_group_role) | **POST** /projects/{project_id}/groups/{group_id}/roles | Assign project role to group
[**list_project_group_role_assignments**](ProjectGroupRoleAssignmentsApi.md#list_project_group_role_assignments) | **GET** /projects/{project_id}/groups/{group_id}/roles | List project group role assignments
[**retrieve_project_group_role**](ProjectGroupRoleAssignmentsApi.md#retrieve_project_group_role) | **GET** /projects/{project_id}/groups/{group_id}/roles/{role_id} | Retrieve project group role
[**unassign_project_group_role**](ProjectGroupRoleAssignmentsApi.md#unassign_project_group_role) | **DELETE** /projects/{project_id}/groups/{group_id}/roles/{role_id} | Unassign project role from group



## assign_project_group_role

> models::GroupRoleAssignment assign_project_group_role(project_id, group_id, public_assign_organization_group_role_body, options)
Assign project role to group

Assigns a project role to a group within a project.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**group_id** | **String** | The ID of the group that should receive the project role. | [required] |
**public_assign_organization_group_role_body** | [**PublicAssignOrganizationGroupRoleBody**](PublicAssignOrganizationGroupRoleBody.md) | Identifies the project role to assign to the group. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
assign_project_group_role().configuration(&configuration).project_id(..).group_id(..).public_assign_organization_group_role_body(..).options(&options).call().await
```

### Return type

[**models::GroupRoleAssignment**](GroupRoleAssignment.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_group_role_assignments

> models::RoleListResource list_project_group_role_assignments(project_id, group_id, limit, after, order, options)
List project group role assignments

Lists the project roles assigned to a group within a project.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to inspect. | [required] |
**group_id** | **String** | The ID of the group to inspect. | [required] |
**limit** | Option<**i64**> | A limit on the number of project role assignments to return. |  |
**after** | Option<**String**> | Cursor for pagination. Provide the value from the previous response's `next` field to continue listing project roles. |  |
**order** | Option<**String**> | Sort order for the returned project roles. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_project_group_role_assignments().configuration(&configuration).project_id(..).group_id(..).options(&options).call().await
```

### Return type

[**models::RoleListResource**](RoleListResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_project_group_role

> models::AssignedRoleDetails retrieve_project_group_role(project_id, group_id, role_id, options)
Retrieve project group role

Retrieves a project role assigned to a group.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to inspect. | [required] |
**group_id** | **String** | The ID of the group to inspect. | [required] |
**role_id** | **String** | The ID of the project role to retrieve for the group. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_project_group_role().configuration(&configuration).project_id(..).group_id(..).role_id(..).options(&options).call().await
```

### Return type

[**models::AssignedRoleDetails**](AssignedRoleDetails.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unassign_project_group_role

> models::DeletedRoleAssignmentResource unassign_project_group_role(project_id, group_id, role_id, options)
Unassign project role from group

Unassigns a project role from a group within a project.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to modify. | [required] |
**group_id** | **String** | The ID of the group whose project role assignment should be removed. | [required] |
**role_id** | **String** | The ID of the project role to remove from the group. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
unassign_project_group_role().configuration(&configuration).project_id(..).group_id(..).role_id(..).options(&options).call().await
```

### Return type

[**models::DeletedRoleAssignmentResource**](DeletedRoleAssignmentResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

