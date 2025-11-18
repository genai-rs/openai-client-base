# \ProjectUserRoleAssignmentsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_project_user_role**](ProjectUserRoleAssignmentsApi.md#assign_project_user_role) | **POST** /projects/{project_id}/users/{user_id}/roles | Assign project role to user
[**list_project_user_role_assignments**](ProjectUserRoleAssignmentsApi.md#list_project_user_role_assignments) | **GET** /projects/{project_id}/users/{user_id}/roles | List project user role assignments
[**unassign_project_user_role**](ProjectUserRoleAssignmentsApi.md#unassign_project_user_role) | **DELETE** /projects/{project_id}/users/{user_id}/roles/{role_id} | Unassign project role from user



## assign_project_user_role

> models::UserRoleAssignment assign_project_user_role(project_id, user_id, public_assign_organization_group_role_body)
Assign project role to user

Assigns a project role to a user within a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**user_id** | **String** | The ID of the user that should receive the project role. | [required] |
**public_assign_organization_group_role_body** | [**PublicAssignOrganizationGroupRoleBody**](PublicAssignOrganizationGroupRoleBody.md) | Identifies the project role to assign to the user. | [required] |

### Return type

[**models::UserRoleAssignment**](UserRoleAssignment.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_user_role_assignments

> models::RoleListResource list_project_user_role_assignments(project_id, user_id, limit, after, order)
List project user role assignments

Lists the project roles assigned to a user within a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to inspect. | [required] |
**user_id** | **String** | The ID of the user to inspect. | [required] |
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


## unassign_project_user_role

> models::DeletedRoleAssignmentResource unassign_project_user_role(project_id, user_id, role_id)
Unassign project role from user

Unassigns a project role from a user within a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to modify. | [required] |
**user_id** | **String** | The ID of the user whose project role assignment should be removed. | [required] |
**role_id** | **String** | The ID of the project role to remove from the user. | [required] |

### Return type

[**models::DeletedRoleAssignmentResource**](DeletedRoleAssignmentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

