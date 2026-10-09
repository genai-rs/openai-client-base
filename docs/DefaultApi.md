# \DefaultApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**agent_environment_expired_post**](DefaultApi.md#agent_environment_expired_post) | **POST** /agent_environment_expired | 
[**agent_environment_failed_post**](DefaultApi.md#agent_environment_failed_post) | **POST** /agent_environment_failed | 
[**agent_environment_ready_post**](DefaultApi.md#agent_environment_ready_post) | **POST** /agent_environment_ready | 
[**agent_environment_suspended_post**](DefaultApi.md#agent_environment_suspended_post) | **POST** /agent_environment_suspended | 
[**agent_session_action_required_post**](DefaultApi.md#agent_session_action_required_post) | **POST** /agent_session_action_required | 
[**agent_session_created_post**](DefaultApi.md#agent_session_created_post) | **POST** /agent_session_created | 
[**agent_session_failed_post**](DefaultApi.md#agent_session_failed_post) | **POST** /agent_session_failed | 
[**agent_session_idle_post**](DefaultApi.md#agent_session_idle_post) | **POST** /agent_session_idle | 
[**agent_session_in_progress_post**](DefaultApi.md#agent_session_in_progress_post) | **POST** /agent_session_in_progress | 
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
[**live_call_incoming_post**](DefaultApi.md#live_call_incoming_post) | **POST** /live_call_incoming | 
[**live_transport_incoming_post**](DefaultApi.md#live_transport_incoming_post) | **POST** /live_transport_incoming | 
[**realtime_call_incoming_post**](DefaultApi.md#realtime_call_incoming_post) | **POST** /realtime_call_incoming | 
[**response_cancelled_post**](DefaultApi.md#response_cancelled_post) | **POST** /response_cancelled | 
[**response_completed_post**](DefaultApi.md#response_completed_post) | **POST** /response_completed | 
[**response_failed_post**](DefaultApi.md#response_failed_post) | **POST** /response_failed | 
[**response_incomplete_post**](DefaultApi.md#response_incomplete_post) | **POST** /response_incomplete | 
[**safety_alert_created_post**](DefaultApi.md#safety_alert_created_post) | **POST** /safety_alert_created | 
[**safety_deactivation_issued_post**](DefaultApi.md#safety_deactivation_issued_post) | **POST** /safety_deactivation_issued | 
[**safety_org_alert_created_post**](DefaultApi.md#safety_org_alert_created_post) | **POST** /safety_org_alert_created | 
[**safety_warning_issued_post**](DefaultApi.md#safety_warning_issued_post) | **POST** /safety_warning_issued | 



## agent_environment_expired_post

> agent_environment_expired_post(webhook_agent_environment_expired)


Sent when an agent environment expires and can no longer resume from a snapshot. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_agent_environment_expired** | Option<[**WebhookAgentEnvironmentExpired**](WebhookAgentEnvironmentExpired.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## agent_environment_failed_post

> agent_environment_failed_post(webhook_agent_environment_failed)


Sent when setup fails for a prewarmed OpenAI-hosted environment before it is attached to a session. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_agent_environment_failed** | Option<[**WebhookAgentEnvironmentFailed**](WebhookAgentEnvironmentFailed.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## agent_environment_ready_post

> agent_environment_ready_post(webhook_agent_environment_ready)


Sent when a prewarmed OpenAI-hosted environment finishes setup before being attached to a session. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_agent_environment_ready** | Option<[**WebhookAgentEnvironmentReady**](WebhookAgentEnvironmentReady.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## agent_environment_suspended_post

> agent_environment_suspended_post(webhook_agent_environment_suspended)


Sent when an agent environment is suspended and can resume from a snapshot. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_agent_environment_suspended** | Option<[**WebhookAgentEnvironmentSuspended**](WebhookAgentEnvironmentSuspended.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## agent_session_action_required_post

> agent_session_action_required_post(webhook_agent_session_action_required)


Sent when an agent session requires an action. Retrieve the session for action details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_agent_session_action_required** | Option<[**WebhookAgentSessionActionRequired**](WebhookAgentSessionActionRequired.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## agent_session_created_post

> agent_session_created_post(webhook_agent_session_created)


Sent when an agent session is created. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_agent_session_created** | Option<[**WebhookAgentSessionCreated**](WebhookAgentSessionCreated.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## agent_session_failed_post

> agent_session_failed_post(webhook_agent_session_failed)


Sent when an agent session fails. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_agent_session_failed** | Option<[**WebhookAgentSessionFailed**](WebhookAgentSessionFailed.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## agent_session_idle_post

> agent_session_idle_post(webhook_agent_session_idle)


Sent when an agent session becomes idle. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_agent_session_idle** | Option<[**WebhookAgentSessionIdle**](WebhookAgentSessionIdle.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## agent_session_in_progress_post

> agent_session_in_progress_post(webhook_agent_session_in_progress)


Sent when an agent session enters the in-progress state. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_agent_session_in_progress** | Option<[**WebhookAgentSessionInProgress**](WebhookAgentSessionInProgress.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## live_call_incoming_post

> live_call_incoming_post(webhook_live_call_incoming)


Deprecated: use `live.transport.incoming`. Retained only for existing subscriptions. Sent when an incoming API SIP session is available for Live acceptance. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_live_call_incoming** | Option<[**WebhookLiveCallIncoming**](WebhookLiveCallIncoming.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## live_transport_incoming_post

> live_transport_incoming_post(webhook_live_transport_incoming)


Sent when an incoming API SIP session is available for Live acceptance. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_live_transport_incoming** | Option<[**WebhookLiveTransportIncoming**](WebhookLiveTransportIncoming.md)> | The event payload sent by the API. |  |

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


Sent when an incoming API SIP session is available for Realtime acceptance. 

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


## safety_alert_created_post

> safety_alert_created_post(webhook_safety_alert_created)


Sent when an approved safety alert is available for an API project. Retrieve the alert with a project API key granted `api.safety.alerts.read`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_safety_alert_created** | Option<[**WebhookSafetyAlertCreated**](WebhookSafetyAlertCreated.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## safety_deactivation_issued_post

> safety_deactivation_issued_post(webhook_safety_deactivation_issued)


Sent when a deactivation is issued for a safety identifier in your organization. Retrieve the case details with `GET /v1/safety/cases/{id}` using `data.id`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_safety_deactivation_issued** | Option<[**WebhookSafetyDeactivationIssued**](WebhookSafetyDeactivationIssued.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## safety_org_alert_created_post

> safety_org_alert_created_post(webhook_safety_org_alert_created)


Sent when an approved safety alert is available for an enterprise workspace. Retrieve the alert from `https://api.chatgpt.com/v1/safety/alerts/{id}` with an administrator API key for the workspace's backing organization granted `chatgpt.enterprise.safety_alerts.read`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_safety_org_alert_created** | Option<[**WebhookSafetyOrgAlertCreated**](WebhookSafetyOrgAlertCreated.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## safety_warning_issued_post

> safety_warning_issued_post(webhook_safety_warning_issued)


Sent when a warning is issued for a safety identifier in your organization. Retrieve the case details with `GET /v1/safety/cases/{id}` using `data.id`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_safety_warning_issued** | Option<[**WebhookSafetyWarningIssued**](WebhookSafetyWarningIssued.md)> | The event payload sent by the API. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

