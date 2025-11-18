# \GroupOrganizationRoleAssignmentsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_group_role**](GroupOrganizationRoleAssignmentsApi.md#assign_group_role) | **POST** /organization/groups/{group_id}/roles | Assign organization role to group
[**list_group_role_assignments**](GroupOrganizationRoleAssignmentsApi.md#list_group_role_assignments) | **GET** /organization/groups/{group_id}/roles | List group organization role assignments
[**unassign_group_role**](GroupOrganizationRoleAssignmentsApi.md#unassign_group_role) | **DELETE** /organization/groups/{group_id}/roles/{role_id} | Unassign organization role from group



## assign_group_role

> models::GroupRoleAssignment assign_group_role(group_id, public_assign_organization_group_role_body)
Assign organization role to group

Assigns an organization role to a group within the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group that should receive the organization role. | [required] |
**public_assign_organization_group_role_body** | [**PublicAssignOrganizationGroupRoleBody**](PublicAssignOrganizationGroupRoleBody.md) | Identifies the organization role to assign to the group. | [required] |

### Return type

[**models::GroupRoleAssignment**](GroupRoleAssignment.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_role_assignments

> models::RoleListResource list_group_role_assignments(group_id, limit, after, order)
List group organization role assignments

Lists the organization roles assigned to a group within the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group whose organization role assignments you want to list. | [required] |
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


## unassign_group_role

> models::DeletedRoleAssignmentResource unassign_group_role(group_id, role_id)
Unassign organization role from group

Unassigns an organization role from a group within the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group to modify. | [required] |
**role_id** | **String** | The ID of the organization role to remove from the group. | [required] |

### Return type

[**models::DeletedRoleAssignmentResource**](DeletedRoleAssignmentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

