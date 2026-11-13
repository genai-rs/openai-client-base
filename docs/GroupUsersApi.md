# \GroupUsersApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_user**](GroupUsersApi.md#add_group_user) | **POST** /organization/groups/{group_id}/users | Add group user
[**list_group_users**](GroupUsersApi.md#list_group_users) | **GET** /organization/groups/{group_id}/users | List group users
[**remove_group_user**](GroupUsersApi.md#remove_group_user) | **DELETE** /organization/groups/{group_id}/users/{user_id} | Remove group user
[**retrieve_group_user**](GroupUsersApi.md#retrieve_group_user) | **GET** /organization/groups/{group_id}/users/{user_id} | Retrieve group user



## add_group_user

> models::GroupUserAssignment add_group_user(group_id, create_group_user_body, options)
Add group user

Adds a user to a group.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group to update. | [required] |
**create_group_user_body** | [**CreateGroupUserBody**](CreateGroupUserBody.md) | Identifies the user that should be added to the group. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
add_group_user().configuration(&configuration).group_id(..).create_group_user_body(..).options(&options).call().await
```

### Return type

[**models::GroupUserAssignment**](GroupUserAssignment.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_users

> models::UserListResource list_group_users(group_id, limit, after, order, options)
List group users

Lists the users assigned to a group.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group to inspect. | [required] |
**limit** | Option<**i64**> | A limit on the number of users to be returned. Limit can range between 0 and 1000, and the default is 100.  Defaults to `100`. |  |[default to 100]
**after** | Option<**String**> | A cursor for use in pagination. Provide the ID of the last user from the previous list response to retrieve the next page.  |  |
**order** | Option<**String**> | Specifies the sort order of users in the list.  Defaults to `\"desc\"`. |  |[default to desc]
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_group_users().configuration(&configuration).group_id(..).options(&options).call().await
```

### Return type

[**models::UserListResource**](UserListResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_group_user

> models::GroupUserDeletedResource remove_group_user(group_id, user_id, options)
Remove group user

Removes a user from a group.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group to update. | [required] |
**user_id** | **String** | The ID of the user to remove from the group. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
remove_group_user().configuration(&configuration).group_id(..).user_id(..).options(&options).call().await
```

### Return type

[**models::GroupUserDeletedResource**](GroupUserDeletedResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_group_user

> models::GroupMemberUser retrieve_group_user(group_id, user_id, options)
Retrieve group user

Retrieves a user in a group.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group to inspect. | [required] |
**user_id** | **String** | The ID of the user to retrieve from the group. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_group_user().configuration(&configuration).group_id(..).user_id(..).options(&options).call().await
```

### Return type

[**models::GroupMemberUser**](GroupMemberUser.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

