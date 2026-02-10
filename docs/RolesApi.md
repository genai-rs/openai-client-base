# \RolesApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project_role**](RolesApi.md#create_project_role) | **POST** /projects/{project_id}/roles | Creates a custom role for a project.
[**create_role**](RolesApi.md#create_role) | **POST** /organization/roles | Creates a custom role for the organization.
[**delete_project_role**](RolesApi.md#delete_project_role) | **DELETE** /projects/{project_id}/roles/{role_id} | Deletes a custom role from a project.
[**delete_role**](RolesApi.md#delete_role) | **DELETE** /organization/roles/{role_id} | Deletes a custom role from the organization.
[**list_project_roles**](RolesApi.md#list_project_roles) | **GET** /projects/{project_id}/roles | Lists the roles configured for a project.
[**list_roles**](RolesApi.md#list_roles) | **GET** /organization/roles | Lists the roles configured for the organization.
[**update_project_role**](RolesApi.md#update_project_role) | **POST** /projects/{project_id}/roles/{role_id} | Updates an existing project role.
[**update_role**](RolesApi.md#update_role) | **POST** /organization/roles/{role_id} | Updates an existing organization role.



## create_project_role

> models::Role create_project_role(project_id, public_create_organization_role_body)
Creates a custom role for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**public_create_organization_role_body** | [**PublicCreateOrganizationRoleBody**](PublicCreateOrganizationRoleBody.md) | Parameters for the project role you want to create. | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_role

> models::Role create_role(public_create_organization_role_body)
Creates a custom role for the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_create_organization_role_body** | [**PublicCreateOrganizationRoleBody**](PublicCreateOrganizationRoleBody.md) | Parameters for the role you want to create. | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_role

> models::RoleDeletedResource delete_project_role(project_id, role_id)
Deletes a custom role from a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**role_id** | **String** | The ID of the role to delete. | [required] |

### Return type

[**models::RoleDeletedResource**](RoleDeletedResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role

> models::RoleDeletedResource delete_role(role_id)
Deletes a custom role from the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The ID of the role to delete. | [required] |

### Return type

[**models::RoleDeletedResource**](RoleDeletedResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_roles

> models::PublicRoleListResource list_project_roles(project_id, limit, after, order)
Lists the roles configured for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to inspect. | [required] |
**limit** | Option<**i32**> | A limit on the number of roles to return. Defaults to 1000. |  |[default to 1000]
**after** | Option<**String**> | Cursor for pagination. Provide the value from the previous response's `next` field to continue listing roles. |  |
**order** | Option<**String**> | Sort order for the returned roles. |  |[default to asc]

### Return type

[**models::PublicRoleListResource**](PublicRoleListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_roles

> models::PublicRoleListResource list_roles(limit, after, order)
Lists the roles configured for the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | A limit on the number of roles to return. Defaults to 1000. |  |[default to 1000]
**after** | Option<**String**> | Cursor for pagination. Provide the value from the previous response's `next` field to continue listing roles. |  |
**order** | Option<**String**> | Sort order for the returned roles. |  |[default to asc]

### Return type

[**models::PublicRoleListResource**](PublicRoleListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_role

> models::Role update_project_role(project_id, role_id, public_update_organization_role_body)
Updates an existing project role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**role_id** | **String** | The ID of the role to update. | [required] |
**public_update_organization_role_body** | [**PublicUpdateOrganizationRoleBody**](PublicUpdateOrganizationRoleBody.md) | Fields to update on the project role. | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role

> models::Role update_role(role_id, public_update_organization_role_body)
Updates an existing organization role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The ID of the role to update. | [required] |
**public_update_organization_role_body** | [**PublicUpdateOrganizationRoleBody**](PublicUpdateOrganizationRoleBody.md) | Fields to update on the role. | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

