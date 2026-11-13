# \ModerationsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_moderation**](ModerationsApi.md#create_moderation) | **POST** /moderations | Create moderation



## create_moderation

> models::CreateModerationResponse create_moderation(create_moderation_request, options)
Create moderation

Classifies if text and/or image inputs are potentially harmful. Learn more in the [moderation guide](https://developers.openai.com/api/docs/guides/moderation). 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_moderation_request** | [**CreateModerationRequest**](CreateModerationRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_moderation().configuration(&configuration).create_moderation_request(..).options(&options).call().await
```

### Return type

[**models::CreateModerationResponse**](CreateModerationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

