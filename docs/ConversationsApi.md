# \ConversationsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_conversation**](ConversationsApi.md#create_conversation) | **POST** /conversations | Create a conversation.
[**create_conversation_items**](ConversationsApi.md#create_conversation_items) | **POST** /conversations/{conversation_id}/items | Create items in a conversation with the given ID.
[**delete_conversation**](ConversationsApi.md#delete_conversation) | **DELETE** /conversations/{conversation_id} | Delete a conversation. Items in the conversation will not be deleted.
[**delete_conversation_item**](ConversationsApi.md#delete_conversation_item) | **DELETE** /conversations/{conversation_id}/items/{item_id} | Delete an item from a conversation with the given IDs.
[**get_conversation**](ConversationsApi.md#get_conversation) | **GET** /conversations/{conversation_id} | Get a conversation
[**get_conversation_item**](ConversationsApi.md#get_conversation_item) | **GET** /conversations/{conversation_id}/items/{item_id} | Get a single item from a conversation with the given IDs.
[**list_conversation_items**](ConversationsApi.md#list_conversation_items) | **GET** /conversations/{conversation_id}/items | List all items for a conversation with the given ID.
[**update_conversation**](ConversationsApi.md#update_conversation) | **POST** /conversations/{conversation_id} | Update a conversation



## create_conversation

> models::ConversationResource create_conversation(create_conversation_body)
Create a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_conversation_body** | Option<[**CreateConversationBody**](CreateConversationBody.md)> |  |  |

### Return type

[**models::ConversationResource**](ConversationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_conversation_items

> models::ConversationItemList create_conversation_items(conversation_id, create_conversation_items_request, include)
Create items in a conversation with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | The ID of the conversation to add the item to. | [required] |
**create_conversation_items_request** | [**CreateConversationItemsRequest**](CreateConversationItemsRequest.md) |  | [required] |
**include** | Option<[**Vec<models::IncludeEnum>**](models::IncludeEnum.md)> | Additional fields to include in the response. See the `include` parameter for [listing Conversation items above](/docs/api-reference/conversations/list-items#conversations_list_items-include) for more information.  |  |

### Return type

[**models::ConversationItemList**](ConversationItemList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversation

> models::DeletedConversationResource delete_conversation(conversation_id)
Delete a conversation. Items in the conversation will not be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | The ID of the conversation to delete. | [required] |

### Return type

[**models::DeletedConversationResource**](DeletedConversationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversation_item

> models::ConversationResource delete_conversation_item(conversation_id, item_id)
Delete an item from a conversation with the given IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | The ID of the conversation that contains the item. | [required] |
**item_id** | **String** | The ID of the item to delete. | [required] |

### Return type

[**models::ConversationResource**](ConversationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation

> models::ConversationResource get_conversation(conversation_id)
Get a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | The ID of the conversation to retrieve. | [required] |

### Return type

[**models::ConversationResource**](ConversationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation_item

> models::ConversationItem get_conversation_item(conversation_id, item_id, include)
Get a single item from a conversation with the given IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | The ID of the conversation that contains the item. | [required] |
**item_id** | **String** | The ID of the item to retrieve. | [required] |
**include** | Option<[**Vec<models::IncludeEnum>**](models::IncludeEnum.md)> | Additional fields to include in the response. See the `include` parameter for [listing Conversation items above](/docs/api-reference/conversations/list-items#conversations_list_items-include) for more information.  |  |

### Return type

[**models::ConversationItem**](ConversationItem.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_conversation_items

> models::ConversationItemList list_conversation_items(conversation_id, limit, order, after, include)
List all items for a conversation with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | The ID of the conversation to list items for. | [required] |
**limit** | Option<**i32**> | A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.  |  |[default to 20]
**order** | Option<**String**> | The order to return the input items in. Default is `desc`. - `asc`: Return the input items in ascending order. - `desc`: Return the input items in descending order.  |  |
**after** | Option<**String**> | An item ID to list items after, used in pagination.  |  |
**include** | Option<[**Vec<models::IncludeEnum>**](models::IncludeEnum.md)> | Specify additional output data to include in the model response. Currently supported values are: - `web_search_call.action.sources`: Include the sources of the web search tool call. - `code_interpreter_call.outputs`: Includes the outputs of python code execution in code interpreter tool call items. - `computer_call_output.output.image_url`: Include image urls from the computer call output. - `file_search_call.results`: Include the search results of the file search tool call. - `message.input_image.image_url`: Include image urls from the input message. - `message.output_text.logprobs`: Include logprobs with assistant messages. - `reasoning.encrypted_content`: Includes an encrypted version of reasoning tokens in reasoning item outputs. This enables reasoning items to be used in multi-turn conversations when using the Responses API statelessly (like when the `store` parameter is set to `false`, or when an organization is enrolled in the zero data retention program). |  |

### Return type

[**models::ConversationItemList**](ConversationItemList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_conversation

> models::ConversationResource update_conversation(conversation_id, update_conversation_body)
Update a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | The ID of the conversation to update. | [required] |
**update_conversation_body** | Option<[**UpdateConversationBody**](UpdateConversationBody.md)> |  |  |

### Return type

[**models::ConversationResource**](ConversationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

