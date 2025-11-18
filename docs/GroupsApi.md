# \GroupsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_group**](GroupsApi.md#create_group) | **POST** /organization/groups | Create group
[**delete_group**](GroupsApi.md#delete_group) | **DELETE** /organization/groups/{group_id} | Delete group
[**list_groups**](GroupsApi.md#list_groups) | **GET** /organization/groups | List groups
[**update_group**](GroupsApi.md#update_group) | **POST** /organization/groups/{group_id} | Update group



## create_group

> models::GroupResponse create_group(create_group_body)
Create group

Creates a new group in the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_group_body** | [**CreateGroupBody**](CreateGroupBody.md) | Parameters for the group you want to create. | [required] |

### Return type

[**models::GroupResponse**](GroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group

> models::GroupDeletedResource delete_group(group_id)
Delete group

Deletes a group from the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group to delete. | [required] |

### Return type

[**models::GroupDeletedResource**](GroupDeletedResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_groups

> models::GroupListResource list_groups(limit, after, order)
List groups

Lists all groups in the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | A limit on the number of groups to be returned. Limit can range between 0 and 1000, and the default is 100.  |  |[default to 100]
**after** | Option<**String**> | A cursor for use in pagination. `after` is a group ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with group_abc, your subsequent call can include `after=group_abc` in order to fetch the next page of the list.  |  |
**order** | Option<**String**> | Specifies the sort order of the returned groups. |  |[default to asc]

### Return type

[**models::GroupListResource**](GroupListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group

> models::GroupResourceWithSuccess update_group(group_id, update_group_body)
Update group

Updates a group's information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group to update. | [required] |
**update_group_body** | [**UpdateGroupBody**](UpdateGroupBody.md) | New attributes to set on the group. | [required] |

### Return type

[**models::GroupResourceWithSuccess**](GroupResourceWithSuccess.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

