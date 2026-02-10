# \ProjectGroupRoleAssignmentsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_project_group_role**](ProjectGroupRoleAssignmentsApi.md#assign_project_group_role) | **POST** /projects/{project_id}/groups/{group_id}/roles | Assigns a project role to a group within a project.
[**list_project_group_role_assignments**](ProjectGroupRoleAssignmentsApi.md#list_project_group_role_assignments) | **GET** /projects/{project_id}/groups/{group_id}/roles | Lists the project roles assigned to a group within a project.
[**unassign_project_group_role**](ProjectGroupRoleAssignmentsApi.md#unassign_project_group_role) | **DELETE** /projects/{project_id}/groups/{group_id}/roles/{role_id} | Unassigns a project role from a group within a project.



## assign_project_group_role

> models::GroupRoleAssignment assign_project_group_role(project_id, group_id, public_assign_organization_group_role_body)
Assigns a project role to a group within a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**group_id** | **String** | The ID of the group that should receive the project role. | [required] |
**public_assign_organization_group_role_body** | [**PublicAssignOrganizationGroupRoleBody**](PublicAssignOrganizationGroupRoleBody.md) | Identifies the project role to assign to the group. | [required] |

### Return type

[**models::GroupRoleAssignment**](GroupRoleAssignment.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_group_role_assignments

> models::RoleListResource list_project_group_role_assignments(project_id, group_id, limit, after, order)
Lists the project roles assigned to a group within a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to inspect. | [required] |
**group_id** | **String** | The ID of the group to inspect. | [required] |
**limit** | Option<**i32**> | A limit on the number of project role assignments to return. |  |
**after** | Option<**String**> | Cursor for pagination. Provide the value from the previous response's `next` field to continue listing project roles. |  |
**order** | Option<**String**> | Sort order for the returned project roles. |  |

### Return type

[**models::RoleListResource**](RoleListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unassign_project_group_role

> models::DeletedRoleAssignmentResource unassign_project_group_role(project_id, group_id, role_id)
Unassigns a project role from a group within a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to modify. | [required] |
**group_id** | **String** | The ID of the group whose project role assignment should be removed. | [required] |
**role_id** | **String** | The ID of the project role to remove from the group. | [required] |

### Return type

[**models::DeletedRoleAssignmentResource**](DeletedRoleAssignmentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

