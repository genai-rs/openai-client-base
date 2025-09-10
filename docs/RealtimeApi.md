# \RealtimeApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_realtime_client_secret**](RealtimeApi.md#create_realtime_client_secret) | **POST** /realtime/client_secrets | Create client secret
[**create_realtime_session**](RealtimeApi.md#create_realtime_session) | **POST** /realtime/sessions | Create session
[**create_realtime_transcription_session**](RealtimeApi.md#create_realtime_transcription_session) | **POST** /realtime/transcription_sessions | Create transcription session



## create_realtime_client_secret

> models::RealtimeCreateClientSecretResponse create_realtime_client_secret(realtime_create_client_secret_request)
Create client secret

Create a Realtime client secret with an associated session configuration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realtime_create_client_secret_request** | [**RealtimeCreateClientSecretRequest**](RealtimeCreateClientSecretRequest.md) | Create a client secret with the given session configuration. | [required] |

### Return type

[**models::RealtimeCreateClientSecretResponse**](RealtimeCreateClientSecretResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_realtime_session

> models::RealtimeSessionCreateResponse create_realtime_session(realtime_session_create_request)
Create session

Create an ephemeral API token for use in client-side applications with the Realtime API. Can be configured with the same session parameters as the `session.update` client event.  It responds with a session object, plus a `client_secret` key which contains a usable ephemeral API token that can be used to authenticate browser clients for the Realtime API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realtime_session_create_request** | [**RealtimeSessionCreateRequest**](RealtimeSessionCreateRequest.md) | Create an ephemeral API key with the given session configuration. | [required] |

### Return type

[**models::RealtimeSessionCreateResponse**](RealtimeSessionCreateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_realtime_transcription_session

> models::RealtimeTranscriptionSessionCreateResponse create_realtime_transcription_session(realtime_transcription_session_create_request)
Create transcription session

Create an ephemeral API token for use in client-side applications with the Realtime API specifically for realtime transcriptions.  Can be configured with the same session parameters as the `transcription_session.update` client event.  It responds with a session object, plus a `client_secret` key which contains a usable ephemeral API token that can be used to authenticate browser clients for the Realtime API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realtime_transcription_session_create_request** | [**RealtimeTranscriptionSessionCreateRequest**](RealtimeTranscriptionSessionCreateRequest.md) | Create an ephemeral API key with the given session configuration. | [required] |

### Return type

[**models::RealtimeTranscriptionSessionCreateResponse**](RealtimeTranscriptionSessionCreateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

