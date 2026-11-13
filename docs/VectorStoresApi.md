# \VectorStoresApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_vector_store_file_batch**](VectorStoresApi.md#cancel_vector_store_file_batch) | **POST** /vector_stores/{vector_store_id}/file_batches/{batch_id}/cancel | Cancel vector store file batch
[**create_vector_store**](VectorStoresApi.md#create_vector_store) | **POST** /vector_stores | Create vector store
[**create_vector_store_file**](VectorStoresApi.md#create_vector_store_file) | **POST** /vector_stores/{vector_store_id}/files | Create vector store file
[**create_vector_store_file_batch**](VectorStoresApi.md#create_vector_store_file_batch) | **POST** /vector_stores/{vector_store_id}/file_batches | Create vector store file batch
[**delete_vector_store**](VectorStoresApi.md#delete_vector_store) | **DELETE** /vector_stores/{vector_store_id} | Delete vector store
[**delete_vector_store_file**](VectorStoresApi.md#delete_vector_store_file) | **DELETE** /vector_stores/{vector_store_id}/files/{file_id} | Delete vector store file
[**get_vector_store**](VectorStoresApi.md#get_vector_store) | **GET** /vector_stores/{vector_store_id} | Retrieve vector store
[**get_vector_store_file**](VectorStoresApi.md#get_vector_store_file) | **GET** /vector_stores/{vector_store_id}/files/{file_id} | Retrieve vector store file
[**get_vector_store_file_batch**](VectorStoresApi.md#get_vector_store_file_batch) | **GET** /vector_stores/{vector_store_id}/file_batches/{batch_id} | Retrieve vector store file batch
[**list_files_in_vector_store_batch**](VectorStoresApi.md#list_files_in_vector_store_batch) | **GET** /vector_stores/{vector_store_id}/file_batches/{batch_id}/files | List vector store files in a batch
[**list_vector_store_files**](VectorStoresApi.md#list_vector_store_files) | **GET** /vector_stores/{vector_store_id}/files | List vector store files
[**list_vector_stores**](VectorStoresApi.md#list_vector_stores) | **GET** /vector_stores | List vector stores
[**modify_vector_store**](VectorStoresApi.md#modify_vector_store) | **POST** /vector_stores/{vector_store_id} | Modify vector store
[**retrieve_vector_store_file_content**](VectorStoresApi.md#retrieve_vector_store_file_content) | **GET** /vector_stores/{vector_store_id}/files/{file_id}/content | Retrieve vector store file content
[**search_vector_store**](VectorStoresApi.md#search_vector_store) | **POST** /vector_stores/{vector_store_id}/search | Search vector store
[**update_vector_store_file_attributes**](VectorStoresApi.md#update_vector_store_file_attributes) | **POST** /vector_stores/{vector_store_id}/files/{file_id} | Update vector store file attributes



## cancel_vector_store_file_batch

> models::VectorStoreFileBatchObject cancel_vector_store_file_batch(vector_store_id, batch_id, options)
Cancel vector store file batch

Cancel a vector store file batch. This attempts to cancel the processing of files in this batch as soon as possible.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store that the file batch belongs to. | [required] |
**batch_id** | **String** | The ID of the file batch to cancel. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
cancel_vector_store_file_batch().configuration(&configuration).vector_store_id(..).batch_id(..).options(&options).call().await
```

### Return type

[**models::VectorStoreFileBatchObject**](VectorStoreFileBatchObject.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vector_store

> models::VectorStoreObject create_vector_store(create_vector_store_request, options)
Create vector store

Create a vector store.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_vector_store_request** | [**CreateVectorStoreRequest**](CreateVectorStoreRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_vector_store().configuration(&configuration).create_vector_store_request(..).options(&options).call().await
```

### Return type

[**models::VectorStoreObject**](VectorStoreObject.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vector_store_file

> models::VectorStoreFileObject create_vector_store_file(vector_store_id, create_vector_store_file_request, options)
Create vector store file

Create a vector store file by attaching a [File](https://developers.openai.com/api/reference/resources/files) to a [vector store](https://developers.openai.com/api/reference/resources/vector_stores).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store for which to create a File.  | [required] |
**create_vector_store_file_request** | [**CreateVectorStoreFileRequest**](CreateVectorStoreFileRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_vector_store_file().configuration(&configuration).vector_store_id(..).create_vector_store_file_request(..).options(&options).call().await
```

### Return type

[**models::VectorStoreFileObject**](VectorStoreFileObject.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vector_store_file_batch

> models::VectorStoreFileBatchObject create_vector_store_file_batch(vector_store_id, create_vector_store_file_batch_request, options)
Create vector store file batch

Create a vector store file batch.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store for which to create a File Batch.  | [required] |
**create_vector_store_file_batch_request** | [**CreateVectorStoreFileBatchRequest**](CreateVectorStoreFileBatchRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_vector_store_file_batch().configuration(&configuration).vector_store_id(..).create_vector_store_file_batch_request(..).options(&options).call().await
```

### Return type

[**models::VectorStoreFileBatchObject**](VectorStoreFileBatchObject.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vector_store

> models::DeleteVectorStoreResponse delete_vector_store(vector_store_id, options)
Delete vector store

Delete a vector store.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store to delete. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
delete_vector_store().configuration(&configuration).vector_store_id(..).options(&options).call().await
```

### Return type

[**models::DeleteVectorStoreResponse**](DeleteVectorStoreResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vector_store_file

> models::DeleteVectorStoreFileResponse delete_vector_store_file(vector_store_id, file_id, options)
Delete vector store file

Delete a vector store file. This will remove the file from the vector store but the file itself will not be deleted. To delete the file, use the [delete file](https://developers.openai.com/api/reference/resources/files/methods/delete) endpoint.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store that the file belongs to. | [required] |
**file_id** | **String** | The ID of the file to delete. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
delete_vector_store_file().configuration(&configuration).vector_store_id(..).file_id(..).options(&options).call().await
```

### Return type

[**models::DeleteVectorStoreFileResponse**](DeleteVectorStoreFileResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vector_store

> models::VectorStoreObject get_vector_store(vector_store_id, options)
Retrieve vector store

Retrieves a vector store.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store to retrieve. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_vector_store().configuration(&configuration).vector_store_id(..).options(&options).call().await
```

### Return type

[**models::VectorStoreObject**](VectorStoreObject.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vector_store_file

> models::VectorStoreFileObject get_vector_store_file(vector_store_id, file_id, options)
Retrieve vector store file

Retrieves a vector store file.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store that the file belongs to. | [required] |
**file_id** | **String** | The ID of the file being retrieved. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_vector_store_file().configuration(&configuration).vector_store_id(..).file_id(..).options(&options).call().await
```

### Return type

[**models::VectorStoreFileObject**](VectorStoreFileObject.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vector_store_file_batch

> models::VectorStoreFileBatchObject get_vector_store_file_batch(vector_store_id, batch_id, options)
Retrieve vector store file batch

Retrieves a vector store file batch.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store that the file batch belongs to. | [required] |
**batch_id** | **String** | The ID of the file batch being retrieved. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_vector_store_file_batch().configuration(&configuration).vector_store_id(..).batch_id(..).options(&options).call().await
```

### Return type

[**models::VectorStoreFileBatchObject**](VectorStoreFileBatchObject.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_files_in_vector_store_batch

> models::ListVectorStoreFilesResponse list_files_in_vector_store_batch(vector_store_id, batch_id, limit, order, after, before, filter, options)
List vector store files in a batch

Returns a list of vector store files in a batch.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store that the files belong to. | [required] |
**batch_id** | **String** | The ID of the file batch that the files belong to. | [required] |
**limit** | Option<**i64**> | A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<**String**> | Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.  Defaults to `\"desc\"`. |  |[default to desc]
**after** | Option<**String**> | A cursor for use in pagination. `after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with obj_foo, your subsequent call can include after=obj_foo in order to fetch the next page of the list.  |  |
**before** | Option<**String**> | A cursor for use in pagination. `before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with obj_foo, your subsequent call can include before=obj_foo in order to fetch the previous page of the list.  |  |
**filter** | Option<**String**> | Filter by file status. One of `in_progress`, `completed`, `failed`, `cancelled`. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_files_in_vector_store_batch().configuration(&configuration).vector_store_id(..).batch_id(..).options(&options).call().await
```

### Return type

[**models::ListVectorStoreFilesResponse**](ListVectorStoreFilesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_vector_store_files

> models::ListVectorStoreFilesResponse list_vector_store_files(vector_store_id, limit, order, after, before, filter, options)
List vector store files

Returns a list of vector store files.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store that the files belong to. | [required] |
**limit** | Option<**i64**> | A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<**String**> | Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.  Defaults to `\"desc\"`. |  |[default to desc]
**after** | Option<**String**> | A cursor for use in pagination. `after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with obj_foo, your subsequent call can include after=obj_foo in order to fetch the next page of the list.  |  |
**before** | Option<**String**> | A cursor for use in pagination. `before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with obj_foo, your subsequent call can include before=obj_foo in order to fetch the previous page of the list.  |  |
**filter** | Option<**String**> | Filter by file status. One of `in_progress`, `completed`, `failed`, `cancelled`. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_vector_store_files().configuration(&configuration).vector_store_id(..).options(&options).call().await
```

### Return type

[**models::ListVectorStoreFilesResponse**](ListVectorStoreFilesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_vector_stores

> models::ListVectorStoresResponse list_vector_stores(limit, order, after, before, options)
List vector stores

Returns a list of vector stores.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<**String**> | Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.  Defaults to `\"desc\"`. |  |[default to desc]
**after** | Option<**String**> | A cursor for use in pagination. `after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with obj_foo, your subsequent call can include after=obj_foo in order to fetch the next page of the list.  |  |
**before** | Option<**String**> | A cursor for use in pagination. `before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with obj_foo, your subsequent call can include before=obj_foo in order to fetch the previous page of the list.  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_vector_stores().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::ListVectorStoresResponse**](ListVectorStoresResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_vector_store

> models::VectorStoreObject modify_vector_store(vector_store_id, update_vector_store_request, options)
Modify vector store

Modifies a vector store.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store to modify. | [required] |
**update_vector_store_request** | [**UpdateVectorStoreRequest**](UpdateVectorStoreRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
modify_vector_store().configuration(&configuration).vector_store_id(..).update_vector_store_request(..).options(&options).call().await
```

### Return type

[**models::VectorStoreObject**](VectorStoreObject.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_vector_store_file_content

> models::VectorStoreFileContentResponse retrieve_vector_store_file_content(vector_store_id, file_id, options)
Retrieve vector store file content

Retrieve the parsed contents of a vector store file.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store. | [required] |
**file_id** | **String** | The ID of the file within the vector store. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_vector_store_file_content().configuration(&configuration).vector_store_id(..).file_id(..).options(&options).call().await
```

### Return type

[**models::VectorStoreFileContentResponse**](VectorStoreFileContentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_vector_store

> models::VectorStoreSearchResultsPage search_vector_store(vector_store_id, vector_store_search_request, options)
Search vector store

Search a vector store for relevant chunks based on a query and file attributes filter.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store to search. | [required] |
**vector_store_search_request** | [**VectorStoreSearchRequest**](VectorStoreSearchRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
search_vector_store().configuration(&configuration).vector_store_id(..).vector_store_search_request(..).options(&options).call().await
```

### Return type

[**models::VectorStoreSearchResultsPage**](VectorStoreSearchResultsPage.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_vector_store_file_attributes

> models::VectorStoreFileObject update_vector_store_file_attributes(vector_store_id, file_id, update_vector_store_file_attributes_request, options)
Update vector store file attributes

Update attributes on a vector store file.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector_store_id** | **String** | The ID of the vector store the file belongs to. | [required] |
**file_id** | **String** | The ID of the file to update attributes. | [required] |
**update_vector_store_file_attributes_request** | [**UpdateVectorStoreFileAttributesRequest**](UpdateVectorStoreFileAttributesRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
update_vector_store_file_attributes().configuration(&configuration).vector_store_id(..).file_id(..).update_vector_store_file_attributes_request(..).options(&options).call().await
```

### Return type

[**models::VectorStoreFileObject**](VectorStoreFileObject.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

