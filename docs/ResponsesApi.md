# \ResponsesApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**beta_cancel_response**](ResponsesApi.md#beta_cancel_response) | **POST** /responses/{response_id}/cancel?beta=true | Cancel a response
[**beta_compactconversation**](ResponsesApi.md#beta_compactconversation) | **POST** /responses/compact?beta=true | Compact conversation
[**beta_create_response**](ResponsesApi.md#beta_create_response) | **POST** /responses?beta=true | Create a model response
[**beta_delete_response**](ResponsesApi.md#beta_delete_response) | **DELETE** /responses/{response_id}?beta=true | Delete a model response
[**beta_get_response**](ResponsesApi.md#beta_get_response) | **GET** /responses/{response_id}?beta=true | Get a model response
[**beta_getinputtokencounts**](ResponsesApi.md#beta_getinputtokencounts) | **POST** /responses/input_tokens?beta=true | Get input token counts
[**beta_list_input_items**](ResponsesApi.md#beta_list_input_items) | **GET** /responses/{response_id}/input_items?beta=true | List input items
[**cancel_response**](ResponsesApi.md#cancel_response) | **POST** /responses/{response_id}/cancel | Cancel a response
[**compactconversation**](ResponsesApi.md#compactconversation) | **POST** /responses/compact | Compact conversation
[**create_response**](ResponsesApi.md#create_response) | **POST** /responses | Create a model response
[**delete_response**](ResponsesApi.md#delete_response) | **DELETE** /responses/{response_id} | Delete a model response
[**get_response**](ResponsesApi.md#get_response) | **GET** /responses/{response_id} | Get a model response
[**getinputtokencounts**](ResponsesApi.md#getinputtokencounts) | **POST** /responses/input_tokens | Get input token counts
[**list_input_items**](ResponsesApi.md#list_input_items) | **GET** /responses/{response_id}/input_items | List input items



## beta_cancel_response

> models::BetaResponse beta_cancel_response(response_id, openai_beta)
Cancel a response

Cancels a model response with the given ID. Only responses created with the `background` parameter set to `true` can be cancelled. [Learn more](https://developers.openai.com/api/docs/guides/background). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_id** | **String** | The ID of the response to cancel. | [required] |
**openai_beta** | Option<[**Vec<String>**](String.md)> | Optional beta features to enable for this request. |  |

### Return type

[**models::BetaResponse**](BetaResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_compactconversation

> models::BetaCompactResource beta_compactconversation(openai_beta, beta_compact_response_method_public_body)
Compact conversation

Compact a conversation. Returns a compacted response object.  Learn when and how to compact long-running conversations in the [conversation state guide](https://developers.openai.com/api/docs/guides/conversation-state#managing-the-context-window). For ZDR-compatible compaction details, see [Compaction (advanced)](https://developers.openai.com/api/docs/guides/conversation-state#compaction-advanced).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**openai_beta** | Option<[**Vec<String>**](String.md)> | Optional beta features to enable for this request. |  |
**beta_compact_response_method_public_body** | Option<[**BetaCompactResponseMethodPublicBody**](BetaCompactResponseMethodPublicBody.md)> |  |  |

### Return type

[**models::BetaCompactResource**](BetaCompactResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_create_response

> models::BetaResponse beta_create_response(beta_create_response, openai_beta)
Create a model response

Creates a model response. Provide [text](https://developers.openai.com/api/docs/guides/text) or [image](https://developers.openai.com/api/docs/guides/images-vision) inputs to generate [text](https://developers.openai.com/api/docs/guides/text) or [JSON](https://developers.openai.com/api/docs/guides/structured-outputs) outputs. Have the model call your own [custom code](https://developers.openai.com/api/docs/guides/function-calling) or use built-in [tools](https://developers.openai.com/api/docs/guides/tools) like [web search](https://developers.openai.com/api/docs/guides/tools-web-search) or [file search](https://developers.openai.com/api/docs/guides/tools-file-search) to use your own data as input for the model's response. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**beta_create_response** | [**BetaCreateResponse**](BetaCreateResponse.md) |  | [required] |
**openai_beta** | Option<[**Vec<String>**](String.md)> | Optional beta features to enable for this request. |  |

### Return type

[**models::BetaResponse**](BetaResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_delete_response

> beta_delete_response(response_id, openai_beta)
Delete a model response

Deletes a model response with the given ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_id** | **String** | The ID of the response to delete. | [required] |
**openai_beta** | Option<[**Vec<String>**](String.md)> | Optional beta features to enable for this request. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_get_response

> models::BetaResponse beta_get_response(response_id, include, stream, starting_after, include_obfuscation, openai_beta)
Get a model response

Retrieves a model response with the given ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_id** | **String** | The ID of the response to retrieve. | [required] |
**include** | Option<[**Vec<models::BetaIncludeEnum>**](models::BetaIncludeEnum.md)> | Additional fields to include in the response. See the `include` parameter for Response creation above for more information.  |  |
**stream** | Option<**bool**> | If set to true, the model response data will be streamed to the client as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format). See the [Streaming section below](https://developers.openai.com/api/reference/resources/responses/streaming-events) for more information.  |  |
**starting_after** | Option<**i32**> | The sequence number of the event after which to start streaming.  |  |
**include_obfuscation** | Option<**bool**> | When true, stream obfuscation will be enabled. Stream obfuscation adds random characters to an `obfuscation` field on streaming delta events to normalize payload sizes as a mitigation to certain side-channel attacks. These obfuscation fields are included by default, but add a small amount of overhead to the data stream. You can set `include_obfuscation` to false to optimize for bandwidth if you trust the network links between your application and the OpenAI API.  |  |
**openai_beta** | Option<[**Vec<String>**](String.md)> | Optional beta features to enable for this request. |  |

### Return type

[**models::BetaResponse**](BetaResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_getinputtokencounts

> models::BetaTokenCountsResource beta_getinputtokencounts(openai_beta, beta_token_counts_body)
Get input token counts

Returns input token counts of the request.  Returns an object with `object` set to `response.input_tokens` and an `input_tokens` count.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**openai_beta** | Option<[**Vec<String>**](String.md)> | Optional beta features to enable for this request. |  |
**beta_token_counts_body** | Option<[**BetaTokenCountsBody**](BetaTokenCountsBody.md)> |  |  |

### Return type

[**models::BetaTokenCountsResource**](BetaTokenCountsResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## beta_list_input_items

> models::BetaResponseItemList beta_list_input_items(response_id, limit, order, after, include, openai_beta)
List input items

Returns a list of input items for a given response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_id** | **String** | The ID of the response to retrieve input items for. | [required] |
**limit** | Option<**i32**> | A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.  |  |[default to 20]
**order** | Option<**String**> | The order to return the input items in. Default is `desc`. - `asc`: Return the input items in ascending order. - `desc`: Return the input items in descending order.  |  |
**after** | Option<**String**> | An item ID to list items after, used in pagination.  |  |
**include** | Option<[**Vec<models::BetaIncludeEnum>**](models::BetaIncludeEnum.md)> | Additional fields to include in the response. See the `include` parameter for Response creation above for more information.  |  |
**openai_beta** | Option<[**Vec<String>**](String.md)> | Optional beta features to enable for this request. |  |

### Return type

[**models::BetaResponseItemList**](BetaResponseItemList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_response

> models::Response cancel_response(response_id)
Cancel a response

Cancels a model response with the given ID. Only responses created with the `background` parameter set to `true` can be cancelled. [Learn more](https://developers.openai.com/api/docs/guides/background). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_id** | **String** | The ID of the response to cancel. | [required] |

### Return type

[**models::Response**](Response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compactconversation

> models::CompactResource compactconversation(compact_response_method_public_body)
Compact conversation

Compact a conversation. Returns a compacted response object.  Learn when and how to compact long-running conversations in the [conversation state guide](https://developers.openai.com/api/docs/guides/conversation-state#managing-the-context-window). For ZDR-compatible compaction details, see [Compaction (advanced)](https://developers.openai.com/api/docs/guides/conversation-state#compaction-advanced).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compact_response_method_public_body** | Option<[**CompactResponseMethodPublicBody**](CompactResponseMethodPublicBody.md)> |  |  |

### Return type

[**models::CompactResource**](CompactResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_response

> models::Response create_response(create_response)
Create a model response

Creates a model response. Provide [text](https://developers.openai.com/api/docs/guides/text) or [image](https://developers.openai.com/api/docs/guides/images-vision) inputs to generate [text](https://developers.openai.com/api/docs/guides/text) or [JSON](https://developers.openai.com/api/docs/guides/structured-outputs) outputs. Have the model call your own [custom code](https://developers.openai.com/api/docs/guides/function-calling) or use built-in [tools](https://developers.openai.com/api/docs/guides/tools) like [web search](https://developers.openai.com/api/docs/guides/tools-web-search) or [file search](https://developers.openai.com/api/docs/guides/tools-file-search) to use your own data as input for the model's response. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_response** | [**CreateResponse**](CreateResponse.md) |  | [required] |

### Return type

[**models::Response**](Response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_response

> delete_response(response_id)
Delete a model response

Deletes a model response with the given ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_id** | **String** | The ID of the response to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_response

> models::Response get_response(response_id, include, stream, starting_after, include_obfuscation)
Get a model response

Retrieves a model response with the given ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_id** | **String** | The ID of the response to retrieve. | [required] |
**include** | Option<[**Vec<models::IncludeEnum>**](models::IncludeEnum.md)> | Additional fields to include in the response. See the `include` parameter for Response creation above for more information.  |  |
**stream** | Option<**bool**> | If set to true, the model response data will be streamed to the client as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format). See the [Streaming section below](https://developers.openai.com/api/reference/resources/responses/streaming-events) for more information.  |  |
**starting_after** | Option<**i32**> | The sequence number of the event after which to start streaming.  |  |
**include_obfuscation** | Option<**bool**> | When true, stream obfuscation will be enabled. Stream obfuscation adds random characters to an `obfuscation` field on streaming delta events to normalize payload sizes as a mitigation to certain side-channel attacks. These obfuscation fields are included by default, but add a small amount of overhead to the data stream. You can set `include_obfuscation` to false to optimize for bandwidth if you trust the network links between your application and the OpenAI API.  |  |

### Return type

[**models::Response**](Response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getinputtokencounts

> models::TokenCountsResource getinputtokencounts(token_counts_body)
Get input token counts

Returns input token counts of the request.  Returns an object with `object` set to `response.input_tokens` and an `input_tokens` count.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_counts_body** | Option<[**TokenCountsBody**](TokenCountsBody.md)> |  |  |

### Return type

[**models::TokenCountsResource**](TokenCountsResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_input_items

> models::ResponseItemList list_input_items(response_id, limit, order, after, include)
List input items

Returns a list of input items for a given response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_id** | **String** | The ID of the response to retrieve input items for. | [required] |
**limit** | Option<**i32**> | A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.  |  |[default to 20]
**order** | Option<**String**> | The order to return the input items in. Default is `desc`. - `asc`: Return the input items in ascending order. - `desc`: Return the input items in descending order.  |  |
**after** | Option<**String**> | An item ID to list items after, used in pagination.  |  |
**include** | Option<[**Vec<models::IncludeEnum>**](models::IncludeEnum.md)> | Additional fields to include in the response. See the `include` parameter for Response creation above for more information.  |  |

### Return type

[**models::ResponseItemList**](ResponseItemList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

