# \FilesApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_file**](FilesApi.md#create_file) | **POST** /files | Upload file
[**delete_file**](FilesApi.md#delete_file) | **DELETE** /files/{file_id} | Delete file
[**download_file**](FilesApi.md#download_file) | **GET** /files/{file_id}/content | Retrieve file content
[**list_files**](FilesApi.md#list_files) | **GET** /files | List files
[**retrieve_file**](FilesApi.md#retrieve_file) | **GET** /files/{file_id} | Retrieve file



## create_file

> models::OpenAiFile create_file(file, purpose, expires_after)
Upload file

Upload a file that can be used across various endpoints. Individual files can be up to 512 MB, and each project can store up to 2.5 TB of files in total. There is no organization-wide storage limit.  - The Assistants API supports files up to 2 million tokens and of specific   file types. See the [Assistants Tools guide](https://platform.openai.com/docs/assistants/tools) for   details. - The Fine-tuning API only supports `.jsonl` files. The input also has   certain required formats for fine-tuning   [chat](https://platform.openai.com/docs/api-reference/fine-tuning/chat-input) or   [completions](https://platform.openai.com/docs/api-reference/fine-tuning/completions-input) models. - The Batch API only supports `.jsonl` files up to 200 MB in size. The input   also has a specific required   [format](https://platform.openai.com/docs/api-reference/batch/request-input).  Please [contact us](https://help.openai.com/) if you need to increase these storage limits. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | The File object (not file name) to be uploaded.  | [required] |
**purpose** | [**models::FilePurpose**](FilePurpose.md) |  | [required] |
**expires_after** | Option<[**models::FileExpirationAfter**](FileExpirationAfter.md)> |  |  |

### Return type

[**models::OpenAiFile**](OpenAIFile.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_file

> models::DeleteFileResponse delete_file(file_id)
Delete file

Delete a file and remove it from all vector stores.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to use for this request. | [required] |

### Return type

[**models::DeleteFileResponse**](DeleteFileResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_file

> String download_file(file_id)
Retrieve file content

Returns the contents of the specified file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to use for this request. | [required] |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_files

> models::ListFilesResponse list_files(purpose, limit, order, after)
List files

Returns a list of files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purpose** | Option<**String**> | Only return files with the given purpose. |  |
**limit** | Option<**i32**> | A limit on the number of objects to be returned. Limit can range between 1 and 10,000, and the default is 10,000.  |  |[default to 10000]
**order** | Option<**String**> | Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.  |  |[default to desc]
**after** | Option<**String**> | A cursor for use in pagination. `after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with obj_foo, your subsequent call can include after=obj_foo in order to fetch the next page of the list.  |  |

### Return type

[**models::ListFilesResponse**](ListFilesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_file

> models::OpenAiFile retrieve_file(file_id)
Retrieve file

Returns information about a specific file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to use for this request. | [required] |

### Return type

[**models::OpenAiFile**](OpenAIFile.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

