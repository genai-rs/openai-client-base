# \DefaultApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_api_keys_create**](DefaultApi.md#admin_api_keys_create) | **POST** /organization/admin_api_keys | Create admin API key
[**admin_api_keys_delete**](DefaultApi.md#admin_api_keys_delete) | **DELETE** /organization/admin_api_keys/{key_id} | Delete admin API key
[**admin_api_keys_get**](DefaultApi.md#admin_api_keys_get) | **GET** /organization/admin_api_keys/{key_id} | Retrieve admin API key
[**admin_api_keys_list**](DefaultApi.md#admin_api_keys_list) | **GET** /organization/admin_api_keys | List all organization and project API keys.
[**cancel_chat_session_method**](DefaultApi.md#cancel_chat_session_method) | **POST** /chatkit/sessions/{session_id}/cancel | Cancel a ChatKit session
[**create_chat_session_method**](DefaultApi.md#create_chat_session_method) | **POST** /chatkit/sessions | Create a ChatKit session
[**create_container**](DefaultApi.md#create_container) | **POST** /containers | Create container
[**create_container_file**](DefaultApi.md#create_container_file) | **POST** /containers/{container_id}/files | Create container file
[**create_webhook_endpoint**](DefaultApi.md#create_webhook_endpoint) | **POST** /webhook_endpoints | Create Webhook Endpoint
[**createan_ap_ikeyforaserviceaccount**](DefaultApi.md#createan_ap_ikeyforaserviceaccount) | **POST** /organization/projects/{project_id}/service_accounts/{service_account_id}/api_keys | Create an API key for a service account
[**createanexternalstorageconfiguration**](DefaultApi.md#createanexternalstorageconfiguration) | **POST** /organization/external_storage | Create an external storage configuration
[**createcontentprovenancecheck**](DefaultApi.md#createcontentprovenancecheck) | **POST** /content_provenance_checks | Create content provenance check
[**delete_container**](DefaultApi.md#delete_container) | **DELETE** /containers/{container_id} | Delete a container
[**delete_container_file**](DefaultApi.md#delete_container_file) | **DELETE** /containers/{container_id}/files/{file_id} | Delete a container file
[**delete_thread_method**](DefaultApi.md#delete_thread_method) | **DELETE** /chatkit/threads/{thread_id} | Delete a ChatKit thread
[**delete_webhook_endpoint**](DefaultApi.md#delete_webhook_endpoint) | **DELETE** /webhook_endpoints/{webhook_endpoint_id} | Delete Webhook Endpoint
[**deleteanexternalstorageconfiguration**](DefaultApi.md#deleteanexternalstorageconfiguration) | **DELETE** /organization/external_storage/{external_storage_id} | Delete an external storage configuration
[**deleteorganizationspendlimit**](DefaultApi.md#deleteorganizationspendlimit) | **DELETE** /organization/spend_limit | Delete organization spend limit
[**deleteprojectspendlimit**](DefaultApi.md#deleteprojectspendlimit) | **DELETE** /organization/projects/{project_id}/spend_limit | Delete project spend limit
[**get_thread_method**](DefaultApi.md#get_thread_method) | **GET** /chatkit/threads/{thread_id} | Retrieve a ChatKit thread
[**getanexternalstorageconfiguration**](DefaultApi.md#getanexternalstorageconfiguration) | **GET** /organization/external_storage/{external_storage_id} | Get an external storage configuration
[**getorganizationspendlimit**](DefaultApi.md#getorganizationspendlimit) | **GET** /organization/spend_limit | Get organization spend limit
[**getprojectsafetyalert**](DefaultApi.md#getprojectsafetyalert) | **GET** /safety/alerts/{id} | Get project safety alert
[**getprojectspendlimit**](DefaultApi.md#getprojectspendlimit) | **GET** /organization/projects/{project_id}/spend_limit | Get project spend limit
[**getsafetycase**](DefaultApi.md#getsafetycase) | **GET** /safety/cases/{id} | Get safety case
[**list_container_files**](DefaultApi.md#list_container_files) | **GET** /containers/{container_id}/files | List container files
[**list_containers**](DefaultApi.md#list_containers) | **GET** /containers | List containers
[**list_thread_items_method**](DefaultApi.md#list_thread_items_method) | **GET** /chatkit/threads/{thread_id}/items | List ChatKit thread items
[**list_threads_method**](DefaultApi.md#list_threads_method) | **GET** /chatkit/threads | List ChatKit threads
[**list_webhook_endpoints**](DefaultApi.md#list_webhook_endpoints) | **GET** /webhook_endpoints | List Webhook Endpoints
[**list_webhook_event_types**](DefaultApi.md#list_webhook_event_types) | **GET** /webhook_event_types | List Webhook Event Types
[**listexternalstorageconfigurations**](DefaultApi.md#listexternalstorageconfigurations) | **GET** /organization/external_storage | List external storage configurations
[**organization_decisionsusage**](DefaultApi.md#organization_decisionsusage) | **GET** /organization/usage/decisions | Decisions
[**retrieve_container**](DefaultApi.md#retrieve_container) | **GET** /containers/{container_id} | Retrieve container
[**retrieve_container_file**](DefaultApi.md#retrieve_container_file) | **GET** /containers/{container_id}/files/{file_id} | Retrieve container file
[**retrieve_container_file_content**](DefaultApi.md#retrieve_container_file_content) | **GET** /containers/{container_id}/files/{file_id}/content | Retrieve container file content
[**retrieve_webhook_endpoint**](DefaultApi.md#retrieve_webhook_endpoint) | **GET** /webhook_endpoints/{webhook_endpoint_id} | Retrieve Webhook Endpoint
[**rotate_webhook_endpoint_signing_secret**](DefaultApi.md#rotate_webhook_endpoint_signing_secret) | **POST** /webhook_endpoints/{webhook_endpoint_id}/rotate_secret | Rotate Webhook Endpoint Signing Secret
[**test_webhook_endpoint**](DefaultApi.md#test_webhook_endpoint) | **POST** /webhook_endpoints/{webhook_endpoint_id}/test | Test Webhook Endpoint
[**update_webhook_endpoint**](DefaultApi.md#update_webhook_endpoint) | **POST** /webhook_endpoints/{webhook_endpoint_id} | Update Webhook Endpoint
[**updateorganizationspendlimit**](DefaultApi.md#updateorganizationspendlimit) | **POST** /organization/spend_limit | Update organization spend limit
[**updateprojectspendlimit**](DefaultApi.md#updateprojectspendlimit) | **POST** /organization/projects/{project_id}/spend_limit | Update project spend limit
[**validateanexternalstorageconfiguration**](DefaultApi.md#validateanexternalstorageconfiguration) | **POST** /organization/external_storage/{external_storage_id}/validate | Validate an external storage configuration



## admin_api_keys_create

> models::AdminApiKeyCreateResponse admin_api_keys_create(admin_api_keys_create_request)
Create admin API key

Create an organization admin API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**admin_api_keys_create_request** | [**AdminApiKeysCreateRequest**](AdminApiKeysCreateRequest.md) |  | [required] |

### Return type

[**models::AdminApiKeyCreateResponse**](AdminApiKeyCreateResponse.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_api_keys_delete

> models::AdminApiKeysDelete200Response admin_api_keys_delete(key_id)
Delete admin API key

Delete an organization admin API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |

### Return type

[**models::AdminApiKeysDelete200Response**](admin_api_keys_delete_200_response.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_api_keys_get

> models::AdminApiKey admin_api_keys_get(key_id)
Retrieve admin API key

Retrieve a single organization API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |

### Return type

[**models::AdminApiKey**](AdminApiKey.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_api_keys_list

> models::ApiKeyList admin_api_keys_list(after, order, limit)
List all organization and project API keys.

List organization API keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**String**> |  |  |
**order** | Option<**String**> |  |  |[default to asc]
**limit** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::ApiKeyList**](ApiKeyList.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_chat_session_method

> models::ChatSessionResource cancel_chat_session_method(session_id)
Cancel a ChatKit session

Cancel an active ChatKit session and return its most recent metadata.  Cancelling prevents new requests from using the issued client secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | Unique identifier for the ChatKit session to cancel. | [required] |

### Return type

[**models::ChatSessionResource**](ChatSessionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_chat_session_method

> models::ChatSessionResource create_chat_session_method(create_chat_session_body)
Create a ChatKit session

Create a ChatKit session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_chat_session_body** | Option<[**CreateChatSessionBody**](CreateChatSessionBody.md)> |  |  |

### Return type

[**models::ChatSessionResource**](ChatSessionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_container

> models::ContainerResource create_container(create_container_body)
Create container

Create Container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_container_body** | Option<[**CreateContainerBody**](CreateContainerBody.md)> |  |  |

### Return type

[**models::ContainerResource**](ContainerResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_container_file

> models::ContainerFileResource create_container_file(container_id, create_container_file_body)
Create container file

Create a Container File  You can send either a multipart/form-data request with the raw file content, or a JSON request with a file ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**create_container_file_body** | [**CreateContainerFileBody**](CreateContainerFileBody.md) |  | [required] |

### Return type

[**models::ContainerFileResource**](ContainerFileResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_webhook_endpoint

> models::WebhookEndpointWithSecretResource create_webhook_endpoint(public_create_endpoint_body)
Create Webhook Endpoint

Creates a webhook endpoint for the authenticated project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_create_endpoint_body** | Option<[**PublicCreateEndpointBody**](PublicCreateEndpointBody.md)> |  |  |

### Return type

[**models::WebhookEndpointWithSecretResource**](WebhookEndpointWithSecretResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## createan_ap_ikeyforaserviceaccount

> models::ServiceAccountApiKeyBody createan_ap_ikeyforaserviceaccount(project_id, service_account_id, create_project_service_account_api_key_body)
Create an API key for a service account

Creates an API key for a service account in the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project. | [required] |
**service_account_id** | **String** | The ID of the service account. | [required] |
**create_project_service_account_api_key_body** | Option<[**CreateProjectServiceAccountApiKeyBody**](CreateProjectServiceAccountApiKeyBody.md)> | The service account API key create request payload. |  |

### Return type

[**models::ServiceAccountApiKeyBody**](ServiceAccountApiKeyBody.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## createanexternalstorageconfiguration

> models::ExternalStorageResponse createanexternalstorageconfiguration(create_external_storage_body)
Create an external storage configuration

Register one customer-managed external storage configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_external_storage_body** | [**CreateExternalStorageBody**](CreateExternalStorageBody.md) |  | [required] |

### Return type

[**models::ExternalStorageResponse**](ExternalStorageResponse.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## createcontentprovenancecheck

> models::ProvenanceResource createcontentprovenancecheck(file)
Create content provenance check

Check whether an image or audio file contains known OpenAI provenance signals. [Learn more about content provenance](https://developers.openai.com/api/docs/guides/content-provenance).  If `not_detected`, it means the tool did not find supported signals in the uploaded file. The content could still have been generated by OpenAI if the metadata was stripped or has evidence of tampering, the watermark was degraded, it comes from a legacy generation model, or it was created before provenance signals were available. Content could also still be AI-generated by another company's model, which the tool currently does not detect.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | The image or audio file to check for supported OpenAI provenance signals. | [required] |

### Return type

[**models::ProvenanceResource**](ProvenanceResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_container

> models::DeleteContainer200Response delete_container(container_id)
Delete a container

Delete Container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** | The ID of the container to delete. | [required] |

### Return type

[**models::DeleteContainer200Response**](DeleteContainer_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_container_file

> models::DeleteContainerFile200Response delete_container_file(container_id, file_id)
Delete a container file

Delete Container File

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |

### Return type

[**models::DeleteContainerFile200Response**](DeleteContainerFile_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_thread_method

> models::DeletedThreadResource delete_thread_method(thread_id)
Delete a ChatKit thread

Delete a ChatKit thread along with its items and stored attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** | Identifier of the ChatKit thread to delete. | [required] |

### Return type

[**models::DeletedThreadResource**](DeletedThreadResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook_endpoint

> models::DeletedWebhookEndpointResource delete_webhook_endpoint(webhook_endpoint_id)
Delete Webhook Endpoint

Deletes a webhook endpoint for the authenticated project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_endpoint_id** | **String** | The ID of the webhook endpoint to delete. | [required] |

### Return type

[**models::DeletedWebhookEndpointResource**](DeletedWebhookEndpointResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deleteanexternalstorageconfiguration

> models::ExternalStorageDeletedResource deleteanexternalstorageconfiguration(external_storage_id)
Delete an external storage configuration

Disconnect a customer-managed external storage configuration. Removing the project's last configuration restores organization-default retention if customer-managed retention was active. Repeating a deletion also completes any interrupted retention update. Cloud storage is unchanged.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_storage_id** | **String** |  | [required] |

### Return type

[**models::ExternalStorageDeletedResource**](ExternalStorageDeletedResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deleteorganizationspendlimit

> models::OrganizationSpendLimitDeletedResource deleteorganizationspendlimit()
Delete organization spend limit

Delete the organization's hard spend limit.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OrganizationSpendLimitDeletedResource**](OrganizationSpendLimitDeletedResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deleteprojectspendlimit

> models::ProjectSpendLimitDeletedResource deleteprojectspendlimit(project_id)
Delete project spend limit

Delete a project's hard spend limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project whose hard spend limit is being managed. | [required] |

### Return type

[**models::ProjectSpendLimitDeletedResource**](ProjectSpendLimitDeletedResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thread_method

> models::ThreadResource get_thread_method(thread_id)
Retrieve a ChatKit thread

Retrieve a ChatKit thread by its identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** | Identifier of the ChatKit thread to retrieve. | [required] |

### Return type

[**models::ThreadResource**](ThreadResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getanexternalstorageconfiguration

> models::ExternalStorageResponse getanexternalstorageconfiguration(external_storage_id)
Get an external storage configuration

Get one customer-managed external storage configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_storage_id** | **String** |  | [required] |

### Return type

[**models::ExternalStorageResponse**](ExternalStorageResponse.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getorganizationspendlimit

> models::OrganizationSpendLimitResource getorganizationspendlimit()
Get organization spend limit

Get the organization's hard spend limit.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OrganizationSpendLimitResource**](OrganizationSpendLimitResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getprojectsafetyalert

> models::SafetyAlertResource getprojectsafetyalert(id)
Get project safety alert

Get a safety alert belonging to the authenticated API project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Project safety alert ID | [required] |

### Return type

[**models::SafetyAlertResource**](SafetyAlertResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getprojectspendlimit

> models::ProjectSpendLimitResource getprojectspendlimit(project_id)
Get project spend limit

Get a project's hard spend limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project whose hard spend limit is being managed. | [required] |

### Return type

[**models::ProjectSpendLimitResource**](ProjectSpendLimitResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getsafetycase

> models::SafetyCaseResource getsafetycase(id)
Get safety case

Get a safety case by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Safety case ID | [required] |

### Return type

[**models::SafetyCaseResource**](SafetyCaseResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_container_files

> models::ContainerFileListResource list_container_files(container_id, limit, order, after)
List container files

List Container files

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**limit** | Option<**i32**> | A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.  |  |[default to 20]
**order** | Option<**String**> | Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.  |  |[default to desc]
**after** | Option<**String**> | A cursor for use in pagination. `after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with obj_foo, your subsequent call can include after=obj_foo in order to fetch the next page of the list.  |  |

### Return type

[**models::ContainerFileListResource**](ContainerFileListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_containers

> models::ContainerListResource list_containers(limit, order, after, name)
List containers

List Containers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.  |  |[default to 20]
**order** | Option<**String**> | Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.  |  |[default to desc]
**after** | Option<**String**> | A cursor for use in pagination. `after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with obj_foo, your subsequent call can include after=obj_foo in order to fetch the next page of the list.  |  |
**name** | Option<**String**> | Filter results by container name. |  |

### Return type

[**models::ContainerListResource**](ContainerListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_thread_items_method

> models::ThreadItemListResource list_thread_items_method(thread_id, limit, order, after, before)
List ChatKit thread items

List items that belong to a ChatKit thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** | Identifier of the ChatKit thread whose items are requested. | [required] |
**limit** | Option<**i32**> | Maximum number of thread items to return. Defaults to 20. |  |
**order** | Option<[**OrderEnum**](OrderEnum.md)> | Sort order for results by creation time. Defaults to `desc`. |  |
**after** | Option<**String**> | List items created after this thread item ID. Defaults to null for the first page. |  |
**before** | Option<**String**> | List items created before this thread item ID. Defaults to null for the newest results. |  |

### Return type

[**models::ThreadItemListResource**](ThreadItemListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_threads_method

> models::ThreadListResource list_threads_method(limit, order, after, before, user)
List ChatKit threads

List ChatKit threads with optional pagination and user filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of thread items to return. Defaults to 20. |  |
**order** | Option<[**OrderEnum**](OrderEnum.md)> | Sort order for results by creation time. Defaults to `desc`. |  |
**after** | Option<**String**> | List items created after this thread item ID. Defaults to null for the first page. |  |
**before** | Option<**String**> | List items created before this thread item ID. Defaults to null for the newest results. |  |
**user** | Option<**String**> | Filter threads that belong to this user identifier. Defaults to null to return all users. |  |

### Return type

[**models::ThreadListResource**](ThreadListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webhook_endpoints

> models::WebhookEndpointListResource list_webhook_endpoints(limit, after)
List Webhook Endpoints

Returns webhook endpoints for the authenticated project in newest-first order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of webhook endpoints to return. Defaults to 20. |  |
**after** | Option<**String**> | ID of the last webhook endpoint from the previous page. |  |

### Return type

[**models::WebhookEndpointListResource**](WebhookEndpointListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webhook_event_types

> models::WebhookEventTypeListResource list_webhook_event_types()
List Webhook Event Types

Returns webhook event types visible to the authenticated project.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebhookEventTypeListResource**](WebhookEventTypeListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## listexternalstorageconfigurations

> models::ExternalStorageListResource listexternalstorageconfigurations(project_id, after, order, limit)
List external storage configurations

List the organization's customer-managed external storage configurations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | Option<**String**> |  |  |
**after** | Option<**String**> | Return external storage configurations after this ID. |  |
**order** | Option<[**ExternalStorageOrder**](ExternalStorageOrder.md)> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::ExternalStorageListResource**](ExternalStorageListResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_decisionsusage

> models::PublicDecisionsUsageResponseResource organization_decisionsusage(start_time, end_time, bucket_width, limit, page, project_ids, group_by, user_ids, api_key_ids, models, batch)
Decisions

Get Decisions API usage details for the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time** | **i32** | Start time (Unix seconds) of the query time range, inclusive. | [required] |
**end_time** | Option<**i32**> | End time (Unix seconds) of the query time range, exclusive. |  |
**bucket_width** | Option<**String**> | Width of each time bucket in response. Currently `1m`, `1h` and `1d` are supported, default to `1d`. |  |
**limit** | Option<**i32**> | Specifies the number of buckets to return. - `bucket_width=1d`: default: 7, max: 31 - `bucket_width=1h`: default: 24, max: 168 - `bucket_width=1m`: default: 60, max: 1440 |  |
**page** | Option<**String**> | A cursor for use in pagination. Corresponding to the `next_page` field from the previous response. |  |
**project_ids** | Option<[**Vec<String>**](String.md)> | Return only usage for these projects. |  |
**group_by** | Option<[**Vec<models::DecisionsUsageGroupBy>**](Models__DecisionsUsageGroupBy.md)> | Group the usage data by the specified fields. Support fields include `project_id`, `user_id`, `api_key_id`, `model`, `batch`, `service_tier`, `api_source` or any combination of them. When grouped by `api_source`, results use `agents_api` for attributed Agents API activity and `unlabeled` for all other activity. Without source grouping, `api_source` is null. |  |
**user_ids** | Option<[**Vec<String>**](String.md)> | Return only usage for these users. |  |
**api_key_ids** | Option<[**Vec<String>**](String.md)> | Return only usage for these API keys. |  |
**models** | Option<[**Vec<String>**](String.md)> | Return only usage for these models. |  |
**batch** | Option<**bool**> | If `true`, return batch jobs only. If `false`, return non-batch jobs only. By default, return both. |  |

### Return type

[**models::PublicDecisionsUsageResponseResource**](PublicDecisionsUsageResponseResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_container

> models::ContainerResource retrieve_container(container_id)
Retrieve container

Retrieve Container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |

### Return type

[**models::ContainerResource**](ContainerResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_container_file

> models::ContainerFileResource retrieve_container_file(container_id, file_id)
Retrieve container file

Retrieve Container File

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |

### Return type

[**models::ContainerFileResource**](ContainerFileResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_container_file_content

> std::path::PathBuf retrieve_container_file_content(container_id, file_id)
Retrieve container file content

Retrieve Container File Content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_webhook_endpoint

> models::WebhookEndpointBody retrieve_webhook_endpoint(webhook_endpoint_id)
Retrieve Webhook Endpoint

Retrieves a webhook endpoint for the authenticated project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_endpoint_id** | **String** | The ID of the webhook endpoint to retrieve. | [required] |

### Return type

[**models::WebhookEndpointBody**](WebhookEndpointBody.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_webhook_endpoint_signing_secret

> models::WebhookEndpointWithSecretResource rotate_webhook_endpoint_signing_secret(webhook_endpoint_id, public_rotate_secret_body)
Rotate Webhook Endpoint Signing Secret

Rotates the signing secret for a webhook endpoint in the authenticated project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_endpoint_id** | **String** | The ID of the webhook endpoint whose signing secret will be rotated. | [required] |
**public_rotate_secret_body** | Option<[**PublicRotateSecretBody**](PublicRotateSecretBody.md)> |  |  |

### Return type

[**models::WebhookEndpointWithSecretResource**](WebhookEndpointWithSecretResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_webhook_endpoint

> models::WebhookEndpointTestResultResource test_webhook_endpoint(webhook_endpoint_id, public_test_endpoint_body)
Test Webhook Endpoint

Sends a sample event to a webhook endpoint for the authenticated project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_endpoint_id** | **String** | The ID of the webhook endpoint to test. | [required] |
**public_test_endpoint_body** | Option<[**PublicTestEndpointBody**](PublicTestEndpointBody.md)> |  |  |

### Return type

[**models::WebhookEndpointTestResultResource**](WebhookEndpointTestResultResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook_endpoint

> models::WebhookEndpointBody update_webhook_endpoint(webhook_endpoint_id, public_update_endpoint_body)
Update Webhook Endpoint

Updates a webhook endpoint for the authenticated project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_endpoint_id** | **String** | The ID of the webhook endpoint to update. | [required] |
**public_update_endpoint_body** | Option<[**PublicUpdateEndpointBody**](PublicUpdateEndpointBody.md)> |  |  |

### Return type

[**models::WebhookEndpointBody**](WebhookEndpointBody.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## updateorganizationspendlimit

> models::OrganizationSpendLimitResource updateorganizationspendlimit(update_organization_spend_limit_body)
Update organization spend limit

Create or replace the organization's hard spend limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_organization_spend_limit_body** | [**UpdateOrganizationSpendLimitBody**](UpdateOrganizationSpendLimitBody.md) | Parameters for the hard spend limit you want to create or replace. | [required] |

### Return type

[**models::OrganizationSpendLimitResource**](OrganizationSpendLimitResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## updateprojectspendlimit

> models::ProjectSpendLimitResource updateprojectspendlimit(project_id, update_project_spend_limit_body)
Update project spend limit

Create or replace a project's hard spend limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project whose hard spend limit is being managed. | [required] |
**update_project_spend_limit_body** | [**UpdateProjectSpendLimitBody**](UpdateProjectSpendLimitBody.md) | Parameters for the hard spend limit you want to create or replace. | [required] |

### Return type

[**models::ProjectSpendLimitResource**](ProjectSpendLimitResource.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validateanexternalstorageconfiguration

> models::ExternalStorageResponse validateanexternalstorageconfiguration(external_storage_id)
Validate an external storage configuration

Validate one customer-managed external storage configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_storage_id** | **String** |  | [required] |

### Return type

[**models::ExternalStorageResponse**](ExternalStorageResponse.md)

### Authorization

[AdminApiKeyAuth](../README.md#AdminApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

