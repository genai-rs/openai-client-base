# \LiveApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_live_session**](LiveApi.md#accept_live_session) | **POST** /live/sessions/{session_id}/accept | Accept call
[**create_live**](LiveApi.md#create_live) | **POST** /live/sessions | Create session
[**download_live_recording**](LiveApi.md#download_live_recording) | **GET** /live/sessions/{session_id}/content | Download recording
[**fork_live_session**](LiveApi.md#fork_live_session) | **POST** /live/sessions/{session_id}/fork | Fork session
[**hangup_live_session**](LiveApi.md#hangup_live_session) | **POST** /live/sessions/{session_id}/hangup | Hang up session
[**refer_live_session**](LiveApi.md#refer_live_session) | **POST** /live/sessions/{session_id}/refer | Transfer call
[**reject_live_session**](LiveApi.md#reject_live_session) | **POST** /live/sessions/{session_id}/reject | Reject call



## accept_live_session

> accept_live_session(session_id, live_call_accept_request)
Accept call

Accept an incoming SIP call. Supply session with type live, the model, and startup configuration. Before accepting calls, follow the [Live prompting guide](https://developers.openai.com/api/docs/guides/live-prompting) to write frontend conversation instructions and a separate backend prompt. SIP media format is negotiated; omit audio.format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | Opaque Live session identifier from the creation response or incoming-call webhook. Preserve the returned value unchanged, including its prefix. | [required] |
**live_call_accept_request** | [**LiveCallAcceptRequest**](LiveCallAcceptRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_live

> models::LiveCreateResponse create_live(live_create_request)
Create session

Create a Live WebRTC session. Start with the [Live prompting guide](https://developers.openai.com/api/docs/guides/live-prompting).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**live_create_request** | [**LiveCreateRequest**](LiveCreateRequest.md) |  | [required] |

### Return type

[**models::LiveCreateResponse**](LiveCreateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_live_recording

> std::path::PathBuf download_live_recording(session_id)
Download recording

Get Live session content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the stored Live session to download. Use the session ID returned when the session started with storage enabled. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: audio/wav

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fork_live_session

> models::LiveCreateResponse fork_live_session(session_id, live_fork_request)
Fork session

Fork a stored Live session onto a new WebRTC connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the stored Live session to fork. | [required] |
**live_fork_request** | [**LiveForkRequest**](LiveForkRequest.md) |  | [required] |

### Return type

[**models::LiveCreateResponse**](LiveCreateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hangup_live_session

> hangup_live_session(session_id)
Hang up session

End a SIP call identified by session_id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | Opaque Live session identifier from the creation response or incoming-call webhook. Preserve the returned value unchanged, including its prefix. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refer_live_session

> refer_live_session(session_id, live_call_refer_request)
Transfer call

Transfer a SIP call to another destination. Supply a nonblank target_uri for the SIP Refer-To header.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | Opaque Live session identifier from the creation response or incoming-call webhook. Preserve the returned value unchanged, including its prefix. | [required] |
**live_call_refer_request** | [**LiveCallReferRequest**](LiveCallReferRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reject_live_session

> reject_live_session(session_id, live_call_reject_request)
Reject call

Reject an incoming SIP call. Send a required SIP rejection status_code between 300 and 699.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | Opaque Live session identifier from the creation response or incoming-call webhook. Preserve the returned value unchanged, including its prefix. | [required] |
**live_call_reject_request** | [**LiveCallRejectRequest**](LiveCallRejectRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

