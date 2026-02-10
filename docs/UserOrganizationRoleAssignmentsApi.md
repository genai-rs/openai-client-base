# \UserOrganizationRoleAssignmentsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_user_role**](UserOrganizationRoleAssignmentsApi.md#assign_user_role) | **POST** /organization/users/{user_id}/roles | Assigns an organization role to a user within the organization.
[**list_user_role_assignments**](UserOrganizationRoleAssignmentsApi.md#list_user_role_assignments) | **GET** /organization/users/{user_id}/roles | Lists the organization roles assigned to a user within the organization.
[**unassign_user_role**](UserOrganizationRoleAssignmentsApi.md#unassign_user_role) | **DELETE** /organization/users/{user_id}/roles/{role_id} | Unassigns an organization role from a user within the organization.



## assign_user_role

> models::UserRoleAssignment assign_user_role(user_id, public_assign_organization_group_role_body)
Assigns an organization role to a user within the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user that should receive the organization role. | [required] |
**public_assign_organization_group_role_body** | [**PublicAssignOrganizationGroupRoleBody**](PublicAssignOrganizationGroupRoleBody.md) | Identifies the organization role to assign to the user. | [required] |

### Return type

[**models::UserRoleAssignment**](UserRoleAssignment.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_role_assignments

> models::RoleListResource list_user_role_assignments(user_id, limit, after, order)
Lists the organization roles assigned to a user within the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to inspect. | [required] |
**limit** | Option<**i32**> | A limit on the number of organization role assignments to return. |  |
**after** | Option<**String**> | Cursor for pagination. Provide the value from the previous response's `next` field to continue listing organization roles. |  |
**order** | Option<**String**> | Sort order for the returned organization roles. |  |

### Return type

[**models::RoleListResource**](RoleListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unassign_user_role

> models::DeletedRoleAssignmentResource unassign_user_role(user_id, role_id)
Unassigns an organization role from a user within the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to modify. | [required] |
**role_id** | **String** | The ID of the organization role to remove from the user. | [required] |

### Return type

[**models::DeletedRoleAssignmentResource**](DeletedRoleAssignmentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

