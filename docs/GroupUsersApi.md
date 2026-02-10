# \GroupUsersApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_user**](GroupUsersApi.md#add_group_user) | **POST** /organization/groups/{group_id}/users | Adds a user to a group.
[**list_group_users**](GroupUsersApi.md#list_group_users) | **GET** /organization/groups/{group_id}/users | Lists the users assigned to a group.
[**remove_group_user**](GroupUsersApi.md#remove_group_user) | **DELETE** /organization/groups/{group_id}/users/{user_id} | Removes a user from a group.



## add_group_user

> models::GroupUserAssignment add_group_user(group_id, create_group_user_body)
Adds a user to a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group to update. | [required] |
**create_group_user_body** | [**CreateGroupUserBody**](CreateGroupUserBody.md) | Identifies the user that should be added to the group. | [required] |

### Return type

[**models::GroupUserAssignment**](GroupUserAssignment.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_users

> models::UserListResource list_group_users(group_id, limit, after, order)
Lists the users assigned to a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group to inspect. | [required] |
**limit** | Option<**i32**> | A limit on the number of users to be returned. Limit can range between 0 and 1000, and the default is 100.  |  |[default to 100]
**after** | Option<**String**> | A cursor for use in pagination. Provide the ID of the last user from the previous list response to retrieve the next page.  |  |
**order** | Option<**String**> | Specifies the sort order of users in the list. |  |[default to desc]

### Return type

[**models::UserListResource**](UserListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_group_user

> models::GroupUserDeletedResource remove_group_user(group_id, user_id)
Removes a user from a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group to update. | [required] |
**user_id** | **String** | The ID of the user to remove from the group. | [required] |

### Return type

[**models::GroupUserDeletedResource**](GroupUserDeletedResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

