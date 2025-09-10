# \FineTuningApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_fine_tuning_job**](FineTuningApi.md#cancel_fine_tuning_job) | **POST** /fine_tuning/jobs/{fine_tuning_job_id}/cancel | Cancel fine-tuning
[**create_fine_tuning_checkpoint_permission**](FineTuningApi.md#create_fine_tuning_checkpoint_permission) | **POST** /fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions | Create checkpoint permissions
[**create_fine_tuning_job**](FineTuningApi.md#create_fine_tuning_job) | **POST** /fine_tuning/jobs | Create fine-tuning job
[**delete_fine_tuning_checkpoint_permission**](FineTuningApi.md#delete_fine_tuning_checkpoint_permission) | **DELETE** /fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions/{permission_id} | Delete checkpoint permission
[**list_fine_tuning_checkpoint_permissions**](FineTuningApi.md#list_fine_tuning_checkpoint_permissions) | **GET** /fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions | List checkpoint permissions
[**list_fine_tuning_events**](FineTuningApi.md#list_fine_tuning_events) | **GET** /fine_tuning/jobs/{fine_tuning_job_id}/events | List fine-tuning events
[**list_fine_tuning_job_checkpoints**](FineTuningApi.md#list_fine_tuning_job_checkpoints) | **GET** /fine_tuning/jobs/{fine_tuning_job_id}/checkpoints | List fine-tuning checkpoints
[**list_paginated_fine_tuning_jobs**](FineTuningApi.md#list_paginated_fine_tuning_jobs) | **GET** /fine_tuning/jobs | List fine-tuning jobs
[**pause_fine_tuning_job**](FineTuningApi.md#pause_fine_tuning_job) | **POST** /fine_tuning/jobs/{fine_tuning_job_id}/pause | Pause fine-tuning
[**resume_fine_tuning_job**](FineTuningApi.md#resume_fine_tuning_job) | **POST** /fine_tuning/jobs/{fine_tuning_job_id}/resume | Resume fine-tuning
[**retrieve_fine_tuning_job**](FineTuningApi.md#retrieve_fine_tuning_job) | **GET** /fine_tuning/jobs/{fine_tuning_job_id} | Retrieve fine-tuning job
[**run_grader**](FineTuningApi.md#run_grader) | **POST** /fine_tuning/alpha/graders/run | Run grader
[**validate_grader**](FineTuningApi.md#validate_grader) | **POST** /fine_tuning/alpha/graders/validate | Validate grader



## cancel_fine_tuning_job

> models::FineTuningJob cancel_fine_tuning_job(fine_tuning_job_id)
Cancel fine-tuning

Immediately cancel a fine-tune job. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fine_tuning_job_id** | **String** | The ID of the fine-tuning job to cancel.  | [required] |

### Return type

[**models::FineTuningJob**](FineTuningJob.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_fine_tuning_checkpoint_permission

> models::ListFineTuningCheckpointPermissionResponse create_fine_tuning_checkpoint_permission(fine_tuned_model_checkpoint, create_fine_tuning_checkpoint_permission_request)
Create checkpoint permissions

**NOTE:** Calling this endpoint requires an [admin API key](../admin-api-keys).  This enables organization owners to share fine-tuned models with other projects in their organization. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fine_tuned_model_checkpoint** | **String** | The ID of the fine-tuned model checkpoint to create a permission for.  | [required] |
**create_fine_tuning_checkpoint_permission_request** | [**CreateFineTuningCheckpointPermissionRequest**](CreateFineTuningCheckpointPermissionRequest.md) |  | [required] |

### Return type

[**models::ListFineTuningCheckpointPermissionResponse**](ListFineTuningCheckpointPermissionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_fine_tuning_job

> models::FineTuningJob create_fine_tuning_job(create_fine_tuning_job_request)
Create fine-tuning job

Creates a fine-tuning job which begins the process of creating a new model from a given dataset.  Response includes details of the enqueued job including job status and the name of the fine-tuned models once complete.  [Learn more about fine-tuning](https://platform.openai.com/docs/guides/model-optimization) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_fine_tuning_job_request** | [**CreateFineTuningJobRequest**](CreateFineTuningJobRequest.md) |  | [required] |

### Return type

[**models::FineTuningJob**](FineTuningJob.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_fine_tuning_checkpoint_permission

> models::DeleteFineTuningCheckpointPermissionResponse delete_fine_tuning_checkpoint_permission(fine_tuned_model_checkpoint, permission_id)
Delete checkpoint permission

**NOTE:** This endpoint requires an [admin API key](../admin-api-keys).  Organization owners can use this endpoint to delete a permission for a fine-tuned model checkpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fine_tuned_model_checkpoint** | **String** | The ID of the fine-tuned model checkpoint to delete a permission for.  | [required] |
**permission_id** | **String** | The ID of the fine-tuned model checkpoint permission to delete.  | [required] |

### Return type

[**models::DeleteFineTuningCheckpointPermissionResponse**](DeleteFineTuningCheckpointPermissionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_fine_tuning_checkpoint_permissions

> models::ListFineTuningCheckpointPermissionResponse list_fine_tuning_checkpoint_permissions(fine_tuned_model_checkpoint, project_id, after, limit, order)
List checkpoint permissions

**NOTE:** This endpoint requires an [admin API key](../admin-api-keys).  Organization owners can use this endpoint to view all permissions for a fine-tuned model checkpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fine_tuned_model_checkpoint** | **String** | The ID of the fine-tuned model checkpoint to get permissions for.  | [required] |
**project_id** | Option<**String**> | The ID of the project to get permissions for. |  |
**after** | Option<**String**> | Identifier for the last permission ID from the previous pagination request. |  |
**limit** | Option<**i32**> | Number of permissions to retrieve. |  |[default to 10]
**order** | Option<**String**> | The order in which to retrieve permissions. |  |[default to descending]

### Return type

[**models::ListFineTuningCheckpointPermissionResponse**](ListFineTuningCheckpointPermissionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_fine_tuning_events

> models::ListFineTuningJobEventsResponse list_fine_tuning_events(fine_tuning_job_id, after, limit)
List fine-tuning events

Get status updates for a fine-tuning job. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fine_tuning_job_id** | **String** | The ID of the fine-tuning job to get events for.  | [required] |
**after** | Option<**String**> | Identifier for the last event from the previous pagination request. |  |
**limit** | Option<**i32**> | Number of events to retrieve. |  |[default to 20]

### Return type

[**models::ListFineTuningJobEventsResponse**](ListFineTuningJobEventsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_fine_tuning_job_checkpoints

> models::ListFineTuningJobCheckpointsResponse list_fine_tuning_job_checkpoints(fine_tuning_job_id, after, limit)
List fine-tuning checkpoints

List checkpoints for a fine-tuning job. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fine_tuning_job_id** | **String** | The ID of the fine-tuning job to get checkpoints for.  | [required] |
**after** | Option<**String**> | Identifier for the last checkpoint ID from the previous pagination request. |  |
**limit** | Option<**i32**> | Number of checkpoints to retrieve. |  |[default to 10]

### Return type

[**models::ListFineTuningJobCheckpointsResponse**](ListFineTuningJobCheckpointsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_paginated_fine_tuning_jobs

> models::ListPaginatedFineTuningJobsResponse list_paginated_fine_tuning_jobs(after, limit, metadata)
List fine-tuning jobs

List your organization's fine-tuning jobs 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**String**> | Identifier for the last job from the previous pagination request. |  |
**limit** | Option<**i32**> | Number of fine-tuning jobs to retrieve. |  |[default to 20]
**metadata** | Option<[**std::collections::HashMap<String, String>**](String.md)> | Optional metadata filter. To filter, use the syntax `metadata[k]=v`. Alternatively, set `metadata=null` to indicate no metadata.  |  |

### Return type

[**models::ListPaginatedFineTuningJobsResponse**](ListPaginatedFineTuningJobsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause_fine_tuning_job

> models::FineTuningJob pause_fine_tuning_job(fine_tuning_job_id)
Pause fine-tuning

Pause a fine-tune job. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fine_tuning_job_id** | **String** | The ID of the fine-tuning job to pause.  | [required] |

### Return type

[**models::FineTuningJob**](FineTuningJob.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_fine_tuning_job

> models::FineTuningJob resume_fine_tuning_job(fine_tuning_job_id)
Resume fine-tuning

Resume a fine-tune job. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fine_tuning_job_id** | **String** | The ID of the fine-tuning job to resume.  | [required] |

### Return type

[**models::FineTuningJob**](FineTuningJob.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_fine_tuning_job

> models::FineTuningJob retrieve_fine_tuning_job(fine_tuning_job_id)
Retrieve fine-tuning job

Get info about a fine-tuning job.  [Learn more about fine-tuning](https://platform.openai.com/docs/guides/model-optimization) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fine_tuning_job_id** | **String** | The ID of the fine-tuning job.  | [required] |

### Return type

[**models::FineTuningJob**](FineTuningJob.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_grader

> models::RunGraderResponse run_grader(run_grader_request)
Run grader

Run a grader. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_grader_request** | [**RunGraderRequest**](RunGraderRequest.md) |  | [required] |

### Return type

[**models::RunGraderResponse**](RunGraderResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_grader

> models::ValidateGraderResponse validate_grader(validate_grader_request)
Validate grader

Validate a grader. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_grader_request** | [**ValidateGraderRequest**](ValidateGraderRequest.md) |  | [required] |

### Return type

[**models::ValidateGraderResponse**](ValidateGraderResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

