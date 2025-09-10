# \ModerationsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_moderation**](ModerationsApi.md#create_moderation) | **POST** /moderations | Create moderation



## create_moderation

> models::CreateModerationResponse create_moderation(create_moderation_request)
Create moderation

Classifies if text and/or image inputs are potentially harmful. Learn more in the [moderation guide](https://platform.openai.com/docs/guides/moderation). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_moderation_request** | [**CreateModerationRequest**](CreateModerationRequest.md) |  | [required] |

### Return type

[**models::CreateModerationResponse**](CreateModerationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

