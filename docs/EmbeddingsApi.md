# \EmbeddingsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_embedding**](EmbeddingsApi.md#create_embedding) | **POST** /embeddings | Create embeddings



## create_embedding

> models::CreateEmbeddingResponse create_embedding(create_embedding_request, options)
Create embeddings

Creates an embedding vector representing the input text.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_embedding_request** | [**CreateEmbeddingRequest**](CreateEmbeddingRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_embedding().configuration(&configuration).create_embedding_request(..).options(&options).call().await
```

### Return type

[**models::CreateEmbeddingResponse**](CreateEmbeddingResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

