# \DefaultApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_cancelled_post**](DefaultApi.md#batch_cancelled_post) | **POST** /batch_cancelled | 
[**batch_completed_post**](DefaultApi.md#batch_completed_post) | **POST** /batch_completed | 
[**batch_expired_post**](DefaultApi.md#batch_expired_post) | **POST** /batch_expired | 
[**batch_failed_post**](DefaultApi.md#batch_failed_post) | **POST** /batch_failed | 
[**eval_run_canceled_post**](DefaultApi.md#eval_run_canceled_post) | **POST** /eval_run_canceled | 
[**eval_run_failed_post**](DefaultApi.md#eval_run_failed_post) | **POST** /eval_run_failed | 
[**eval_run_succeeded_post**](DefaultApi.md#eval_run_succeeded_post) | **POST** /eval_run_succeeded | 
[**fine_tuning_job_cancelled_post**](DefaultApi.md#fine_tuning_job_cancelled_post) | **POST** /fine_tuning_job_cancelled | 
[**fine_tuning_job_failed_post**](DefaultApi.md#fine_tuning_job_failed_post) | **POST** /fine_tuning_job_failed | 
[**fine_tuning_job_succeeded_post**](DefaultApi.md#fine_tuning_job_succeeded_post) | **POST** /fine_tuning_job_succeeded | 
[**realtime_call_incoming_post**](DefaultApi.md#realtime_call_incoming_post) | **POST** /realtime_call_incoming | 
[**response_cancelled_post**](DefaultApi.md#response_cancelled_post) | **POST** /response_cancelled | 
[**response_completed_post**](DefaultApi.md#response_completed_post) | **POST** /response_completed | 
[**response_failed_post**](DefaultApi.md#response_failed_post) | **POST** /response_failed | 
[**response_incomplete_post**](DefaultApi.md#response_incomplete_post) | **POST** /response_incomplete | 



## batch_cancelled_post

> batch_cancelled_post(webhook_batch_cancelled)


Sent when a batch has been cancelled. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_batch_cancelled** | Option<[**WebhookBatchCancelled**](WebhookBatchCancelled.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_completed_post

> batch_completed_post(webhook_batch_completed)


Sent when a batch has completed processing. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_batch_completed** | Option<[**WebhookBatchCompleted**](WebhookBatchCompleted.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_expired_post

> batch_expired_post(webhook_batch_expired)


Sent when a batch has expired before completion. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_batch_expired** | Option<[**WebhookBatchExpired**](WebhookBatchExpired.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_failed_post

> batch_failed_post(webhook_batch_failed)


Sent when a batch has failed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_batch_failed** | Option<[**WebhookBatchFailed**](WebhookBatchFailed.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eval_run_canceled_post

> eval_run_canceled_post(webhook_eval_run_canceled)


Sent when an eval run has been canceled. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_eval_run_canceled** | Option<[**WebhookEvalRunCanceled**](WebhookEvalRunCanceled.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eval_run_failed_post

> eval_run_failed_post(webhook_eval_run_failed)


Sent when an eval run has failed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_eval_run_failed** | Option<[**WebhookEvalRunFailed**](WebhookEvalRunFailed.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eval_run_succeeded_post

> eval_run_succeeded_post(webhook_eval_run_succeeded)


Sent when an eval run has succeeded. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_eval_run_succeeded** | Option<[**WebhookEvalRunSucceeded**](WebhookEvalRunSucceeded.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fine_tuning_job_cancelled_post

> fine_tuning_job_cancelled_post(webhook_fine_tuning_job_cancelled)


Sent when a fine-tuning job has been cancelled. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_fine_tuning_job_cancelled** | Option<[**WebhookFineTuningJobCancelled**](WebhookFineTuningJobCancelled.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fine_tuning_job_failed_post

> fine_tuning_job_failed_post(webhook_fine_tuning_job_failed)


Sent when a fine-tuning job has failed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_fine_tuning_job_failed** | Option<[**WebhookFineTuningJobFailed**](WebhookFineTuningJobFailed.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fine_tuning_job_succeeded_post

> fine_tuning_job_succeeded_post(webhook_fine_tuning_job_succeeded)


Sent when a fine-tuning job has succeeded. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_fine_tuning_job_succeeded** | Option<[**WebhookFineTuningJobSucceeded**](WebhookFineTuningJobSucceeded.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## realtime_call_incoming_post

> realtime_call_incoming_post(webhook_realtime_call_incoming)


Sent when Realtime API Receives a incoming SIP call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_realtime_call_incoming** | Option<[**WebhookRealtimeCallIncoming**](WebhookRealtimeCallIncoming.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## response_cancelled_post

> response_cancelled_post(webhook_response_cancelled)


Sent when a background response has been cancelled. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_response_cancelled** | Option<[**WebhookResponseCancelled**](WebhookResponseCancelled.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## response_completed_post

> response_completed_post(webhook_response_completed)


Sent when a background response has completed successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_response_completed** | Option<[**WebhookResponseCompleted**](WebhookResponseCompleted.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## response_failed_post

> response_failed_post(webhook_response_failed)


Sent when a background response has failed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_response_failed** | Option<[**WebhookResponseFailed**](WebhookResponseFailed.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## response_incomplete_post

> response_incomplete_post(webhook_response_incomplete)


Sent when a background response is incomplete. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_response_incomplete** | Option<[**WebhookResponseIncomplete**](WebhookResponseIncomplete.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

