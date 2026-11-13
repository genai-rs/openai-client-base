# \CompletionsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_completion**](CompletionsApi.md#create_completion) | **POST** /completions | Create completion
[**create_completion_stream**](CompletionsApi.md#create_completion_stream) | **POST** /completions | Create completion



## create_completion

> models::CreateCompletionResponse create_completion(create_completion_request, options)
Create completion

Creates a completion for the provided prompt and parameters.  Returns a completion object, or a sequence of completion objects if the request is streamed. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_completion_request** | [**CreateCompletionRequest**](CreateCompletionRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_completion().configuration(&configuration).create_completion_request(..).options(&options).call().await
```

### Return type

[**models::CreateCompletionResponse**](CreateCompletionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_completion_stream

> models::CreateCompletionResponse create_completion_stream(create_completion_request, options)
Create completion

Creates a completion for the provided prompt and parameters.  Returns a completion object, or a sequence of completion objects if the request is streamed. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_completion_request** | [**CreateCompletionRequest**](CreateCompletionRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_completion_stream().configuration(&configuration).create_completion_request(..).options(&options).call().await
```

### Return type

[**models::CreateCompletionResponse**](CreateCompletionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/event-stream, application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

