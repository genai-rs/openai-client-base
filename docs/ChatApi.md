# \ChatApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_chat_completion**](ChatApi.md#create_chat_completion) | **POST** /chat/completions | **Starting a new project?** We recommend trying [Responses](/docs/api-reference/responses) to take advantage of the latest OpenAI platform features. Compare [Chat Completions with Responses](/docs/guides/responses-vs-chat-completions?api-mode=responses).  ---  Creates a model response for the given chat conversation. Learn more in the [text generation](/docs/guides/text-generation), [vision](/docs/guides/vision), and [audio](/docs/guides/audio) guides.  Parameter support can differ depending on the model used to generate the response, particularly for newer reasoning models. Parameters that are only supported for reasoning models are noted below. For the current state of unsupported parameters in reasoning models, [refer to the reasoning guide](/docs/guides/reasoning).  Returns a chat completion object, or a streamed sequence of chat completion chunk objects if the request is streamed. 
[**delete_chat_completion**](ChatApi.md#delete_chat_completion) | **DELETE** /chat/completions/{completion_id} | Delete a stored chat completion. Only Chat Completions that have been created with the `store` parameter set to `true` can be deleted. 
[**get_chat_completion**](ChatApi.md#get_chat_completion) | **GET** /chat/completions/{completion_id} | Get a stored chat completion. Only Chat Completions that have been created with the `store` parameter set to `true` will be returned. 
[**get_chat_completion_messages**](ChatApi.md#get_chat_completion_messages) | **GET** /chat/completions/{completion_id}/messages | Get the messages in a stored chat completion. Only Chat Completions that have been created with the `store` parameter set to `true` will be returned. 
[**list_chat_completions**](ChatApi.md#list_chat_completions) | **GET** /chat/completions | List stored Chat Completions. Only Chat Completions that have been stored with the `store` parameter set to `true` will be returned. 
[**update_chat_completion**](ChatApi.md#update_chat_completion) | **POST** /chat/completions/{completion_id} | Modify a stored chat completion. Only Chat Completions that have been created with the `store` parameter set to `true` can be modified. Currently, the only supported modification is to update the `metadata` field. 



## create_chat_completion

> models::CreateChatCompletionResponse create_chat_completion(create_chat_completion_request)
**Starting a new project?** We recommend trying [Responses](/docs/api-reference/responses) to take advantage of the latest OpenAI platform features. Compare [Chat Completions with Responses](/docs/guides/responses-vs-chat-completions?api-mode=responses).  ---  Creates a model response for the given chat conversation. Learn more in the [text generation](/docs/guides/text-generation), [vision](/docs/guides/vision), and [audio](/docs/guides/audio) guides.  Parameter support can differ depending on the model used to generate the response, particularly for newer reasoning models. Parameters that are only supported for reasoning models are noted below. For the current state of unsupported parameters in reasoning models, [refer to the reasoning guide](/docs/guides/reasoning).  Returns a chat completion object, or a streamed sequence of chat completion chunk objects if the request is streamed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_chat_completion_request** | [**CreateChatCompletionRequest**](CreateChatCompletionRequest.md) |  | [required] |

### Return type

[**models::CreateChatCompletionResponse**](CreateChatCompletionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_chat_completion

> models::ChatCompletionDeleted delete_chat_completion(completion_id)
Delete a stored chat completion. Only Chat Completions that have been created with the `store` parameter set to `true` can be deleted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**completion_id** | **String** | The ID of the chat completion to delete. | [required] |

### Return type

[**models::ChatCompletionDeleted**](ChatCompletionDeleted.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_completion

> models::CreateChatCompletionResponse get_chat_completion(completion_id)
Get a stored chat completion. Only Chat Completions that have been created with the `store` parameter set to `true` will be returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**completion_id** | **String** | The ID of the chat completion to retrieve. | [required] |

### Return type

[**models::CreateChatCompletionResponse**](CreateChatCompletionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_completion_messages

> models::ChatCompletionMessageList get_chat_completion_messages(completion_id, after, limit, order)
Get the messages in a stored chat completion. Only Chat Completions that have been created with the `store` parameter set to `true` will be returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**completion_id** | **String** | The ID of the chat completion to retrieve messages from. | [required] |
**after** | Option<**String**> | Identifier for the last message from the previous pagination request. |  |
**limit** | Option<**i32**> | Number of messages to retrieve. |  |[default to 20]
**order** | Option<**String**> | Sort order for messages by timestamp. Use `asc` for ascending order or `desc` for descending order. Defaults to `asc`. |  |[default to asc]

### Return type

[**models::ChatCompletionMessageList**](ChatCompletionMessageList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_chat_completions

> models::ChatCompletionList list_chat_completions(model, metadata, after, limit, order)
List stored Chat Completions. Only Chat Completions that have been stored with the `store` parameter set to `true` will be returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model** | Option<**String**> | The model used to generate the Chat Completions. |  |
**metadata** | Option<**String**> | A list of metadata keys to filter the Chat Completions by. Example:  `metadata[key1]=value1&metadata[key2]=value2`  |  |
**after** | Option<**String**> | Identifier for the last chat completion from the previous pagination request. |  |
**limit** | Option<**i32**> | Number of Chat Completions to retrieve. |  |[default to 20]
**order** | Option<**String**> | Sort order for Chat Completions by timestamp. Use `asc` for ascending order or `desc` for descending order. Defaults to `asc`. |  |[default to asc]

### Return type

[**models::ChatCompletionList**](ChatCompletionList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chat_completion

> models::CreateChatCompletionResponse update_chat_completion(completion_id, update_chat_completion_request)
Modify a stored chat completion. Only Chat Completions that have been created with the `store` parameter set to `true` can be modified. Currently, the only supported modification is to update the `metadata` field. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**completion_id** | **String** | The ID of the chat completion to update. | [required] |
**update_chat_completion_request** | [**UpdateChatCompletionRequest**](UpdateChatCompletionRequest.md) |  | [required] |

### Return type

[**models::CreateChatCompletionResponse**](CreateChatCompletionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

