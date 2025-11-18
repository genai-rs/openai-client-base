# \ProjectGroupsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_project_group**](ProjectGroupsApi.md#add_project_group) | **POST** /organization/projects/{project_id}/groups | Add project group
[**list_project_groups**](ProjectGroupsApi.md#list_project_groups) | **GET** /organization/projects/{project_id}/groups | List project groups
[**remove_project_group**](ProjectGroupsApi.md#remove_project_group) | **DELETE** /organization/projects/{project_id}/groups/{group_id} | Remove project group



## add_project_group

> models::ProjectGroup add_project_group(project_id, invite_project_group_body)
Add project group

Grants a group access to a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**invite_project_group_body** | [**InviteProjectGroupBody**](InviteProjectGroupBody.md) | Identifies the group and role to assign to the project. | [required] |

### Return type

[**models::ProjectGroup**](ProjectGroup.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_groups

> models::ProjectGroupListResource list_project_groups(project_id, limit, after, order)
List project groups

Lists the groups that have access to a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to inspect. | [required] |
**limit** | Option<**i32**> | A limit on the number of project groups to return. Defaults to 20. |  |[default to 20]
**after** | Option<**String**> | Cursor for pagination. Provide the ID of the last group from the previous response to fetch the next page. |  |
**order** | Option<**String**> | Sort order for the returned groups. |  |[default to asc]

### Return type

[**models::ProjectGroupListResource**](ProjectGroupListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_project_group

> models::ProjectGroupDeletedResource remove_project_group(project_id, group_id)
Remove project group

Revokes a group's access to a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to update. | [required] |
**group_id** | **String** | The ID of the group to remove from the project. | [required] |

### Return type

[**models::ProjectGroupDeletedResource**](ProjectGroupDeletedResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

