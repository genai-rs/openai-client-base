# \AgentsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_agent**](AgentsApi.md#create_agent) | **POST** /agents | Create an agent
[**create_agent_environment_file**](AgentsApi.md#create_agent_environment_file) | **POST** /agents/environments/{environment_id}/files | Create an agent environment file
[**create_agent_environment_template**](AgentsApi.md#create_agent_environment_template) | **POST** /agents/environments/templates | Create an agent environment template
[**create_agent_session**](AgentsApi.md#create_agent_session) | **POST** /agents/sessions | Create an agent session
[**create_agent_session_events**](AgentsApi.md#create_agent_session_events) | **POST** /agents/sessions/{session_id}/events | Create agent session input events
[**delete_agent**](AgentsApi.md#delete_agent) | **DELETE** /agents/{agent_id} | Delete an agent
[**delete_agent_environment_template**](AgentsApi.md#delete_agent_environment_template) | **DELETE** /agents/environments/templates/{environment_template_id} | Delete an agent environment template
[**delete_agent_session**](AgentsApi.md#delete_agent_session) | **DELETE** /agents/sessions/{session_id} | Delete an agent session
[**delete_agent_session_artifact**](AgentsApi.md#delete_agent_session_artifact) | **DELETE** /agents/sessions/{session_id}/artifacts/{artifact_id} | Delete an agent session artifact
[**list_agent_environment_files**](AgentsApi.md#list_agent_environment_files) | **GET** /agents/environments/{environment_id}/files | List agent environment files
[**list_agent_environment_templates**](AgentsApi.md#list_agent_environment_templates) | **GET** /agents/environments/templates | List agent environment templates
[**list_agent_session_artifacts**](AgentsApi.md#list_agent_session_artifacts) | **GET** /agents/sessions/{session_id}/artifacts | List agent session artifacts
[**list_agent_session_events**](AgentsApi.md#list_agent_session_events) | **GET** /agents/sessions/{session_id}/events | Stream agent session events
[**list_agent_session_items**](AgentsApi.md#list_agent_session_items) | **GET** /agents/sessions/{session_id}/items | List agent session items
[**list_agent_session_subagent_items**](AgentsApi.md#list_agent_session_subagent_items) | **GET** /agents/sessions/{session_id}/subagents/{subagent_id}/items | List subagent items
[**list_agent_session_subagent_turn_items**](AgentsApi.md#list_agent_session_subagent_turn_items) | **GET** /agents/sessions/{session_id}/subagents/{subagent_id}/turns/{turn_id}/items | List subagent turn items
[**list_agent_session_subagent_turns**](AgentsApi.md#list_agent_session_subagent_turns) | **GET** /agents/sessions/{session_id}/subagents/{subagent_id}/turns | List subagent turns
[**list_agent_session_subagents**](AgentsApi.md#list_agent_session_subagents) | **GET** /agents/sessions/{session_id}/subagents | List session subagents
[**list_agent_session_turns**](AgentsApi.md#list_agent_session_turns) | **GET** /agents/sessions/{session_id}/turns | List agent session turns
[**list_agent_sessions**](AgentsApi.md#list_agent_sessions) | **GET** /agents/sessions | List agent sessions
[**list_agents**](AgentsApi.md#list_agents) | **GET** /agents | List agents
[**retrieve_agent**](AgentsApi.md#retrieve_agent) | **GET** /agents/{agent_id} | Retrieve an agent
[**retrieve_agent_environment**](AgentsApi.md#retrieve_agent_environment) | **GET** /agents/environments/{environment_id} | Retrieve an agent environment
[**retrieve_agent_environment_template**](AgentsApi.md#retrieve_agent_environment_template) | **GET** /agents/environments/templates/{environment_template_id} | Retrieve an agent environment template
[**retrieve_agent_session**](AgentsApi.md#retrieve_agent_session) | **GET** /agents/sessions/{session_id} | Retrieve an agent session
[**retrieve_agent_session_artifact**](AgentsApi.md#retrieve_agent_session_artifact) | **GET** /agents/sessions/{session_id}/artifacts/{artifact_id} | Retrieve an agent session artifact
[**retrieve_agent_session_artifact_content**](AgentsApi.md#retrieve_agent_session_artifact_content) | **GET** /agents/sessions/{session_id}/artifacts/{artifact_id}/content | Retrieve agent session artifact content
[**retrieve_agent_session_subagent**](AgentsApi.md#retrieve_agent_session_subagent) | **GET** /agents/sessions/{session_id}/subagents/{subagent_id} | Retrieve a session subagent
[**retrieve_agent_session_subagent_turn**](AgentsApi.md#retrieve_agent_session_subagent_turn) | **GET** /agents/sessions/{session_id}/subagents/{subagent_id}/turns/{turn_id} | Retrieve a subagent turn
[**retrieve_agent_session_turn**](AgentsApi.md#retrieve_agent_session_turn) | **GET** /agents/sessions/{session_id}/turns/{turn_id} | Retrieve an agent session turn
[**update_agent**](AgentsApi.md#update_agent) | **POST** /agents/{agent_id} | Update an agent
[**update_agent_environment_template**](AgentsApi.md#update_agent_environment_template) | **POST** /agents/environments/templates/{environment_template_id} | Update an agent environment template
[**update_agent_session**](AgentsApi.md#update_agent_session) | **POST** /agents/sessions/{session_id} | Update an agent session



## create_agent

> models::AgentResource create_agent(create_agent_params)
Create an agent

Creates a reusable agent without storing credentials. See [agent configuration](https://developers.openai.com/api/docs/guides/agents-api/configuration).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_agent_params** | Option<[**CreateAgentParams**](CreateAgentParams.md)> |  |  |

### Return type

[**models::AgentResource**](AgentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_environment_file

> models::EnvironmentFileResource create_agent_environment_file(environment_id, hosted_environment_file_param)
Create an agent environment file

Copies inline bytes or a Files API file into a connected execution environment. See [environment files](https://developers.openai.com/api/docs/guides/agents-api/environments/files).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **String** | The ID of the environment. | [required] |
**hosted_environment_file_param** | Option<[**HostedEnvironmentFileParam**](HostedEnvironmentFileParam.md)> |  |  |

### Return type

[**models::EnvironmentFileResource**](EnvironmentFileResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_environment_template

> models::EnvironmentTemplateResource create_agent_environment_template(create_environment_template_params)
Create an agent environment template

Creates reusable environment configuration without returning confidential setup commands or environment values. See [reusing a hosted setup](https://developers.openai.com/api/docs/guides/agents-api/tools#reuse-a-hosted-plugin-setup).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_environment_template_params** | Option<[**CreateEnvironmentTemplateParams**](CreateEnvironmentTemplateParams.md)> |  |  |

### Return type

[**models::EnvironmentTemplateResource**](EnvironmentTemplateResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_session

> models::SessionResource create_agent_session(create_agent_session_params)
Create an agent session

Creates a managed agent session, optionally submits initial input, and returns the session or streams its events when stream is true. See [running sessions](https://developers.openai.com/api/docs/guides/agents-api/sessions).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_agent_session_params** | Option<[**CreateAgentSessionParams**](CreateAgentSessionParams.md)> |  |  |

### Return type

[**models::SessionResource**](SessionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_session_events

> create_agent_session_events(session_id, idempotency_key, create_session_events_params)
Create agent session input events

Submits message, cancellation, or tool-result events to a managed agent session. Cancellation can recover a still-open turn whose backend execution has ended by marking it cancelled and abandoning unpublished outputs. Saved results, published files, and existing terminal outcomes are preserved. HTTP 202 confirms acceptance, not durable completion. See [session events](https://developers.openai.com/api/docs/guides/agents-api/sessions/events).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**idempotency_key** | Option<**String**> | An optional client-generated key that makes retries of submitted messages idempotent. |  |
**create_session_events_params** | Option<[**CreateSessionEventsParams**](CreateSessionEventsParams.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_agent

> models::DeletedAgentResource delete_agent(agent_id)
Delete an agent

Deletes a reusable agent. See [agent configuration](https://developers.openai.com/api/docs/guides/agents-api/configuration).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** | The ID of the reusable agent. | [required] |

### Return type

[**models::DeletedAgentResource**](DeletedAgentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_agent_environment_template

> models::DeletedEnvironmentTemplateResource delete_agent_environment_template(environment_template_id)
Delete an agent environment template

Deletes reusable environment configuration and all confidential template inputs. See [reusing a hosted setup](https://developers.openai.com/api/docs/guides/agents-api/tools#reuse-a-hosted-plugin-setup).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_template_id** | **String** | The ID of the reusable environment template. | [required] |

### Return type

[**models::DeletedEnvironmentTemplateResource**](DeletedEnvironmentTemplateResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_agent_session

> models::DeletedSessionResource delete_agent_session(session_id)
Delete an agent session

Removes a managed agent session from the public API and returns a deletion confirmation. If backend execution has ended, deletion can cancel a still-open public turn and abandon unpublished outputs. Running execution must be cancelled first. Physical cleanup may continue asynchronously. See [managing sessions](https://developers.openai.com/api/docs/guides/agents-api/sessions/manage).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |

### Return type

[**models::DeletedSessionResource**](DeletedSessionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_agent_session_artifact

> models::DeletedSessionArtifactResource delete_agent_session_artifact(session_id, artifact_id)
Delete an agent session artifact

Deletes an immutable session artifact without deleting its live environment file or original Files API object. See [session artifacts](https://developers.openai.com/api/docs/guides/agents-api/environments/files#openai-hosted-artifacts).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session that owns the artifact. | [required] |
**artifact_id** | **String** | The immutable session artifact ID. | [required] |

### Return type

[**models::DeletedSessionArtifactResource**](DeletedSessionArtifactResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_environment_files

> models::EnvironmentFileListResource list_agent_environment_files(environment_id, path, limit, order, page)
List agent environment files

Lists live files on a connected execution environment with optional directory filtering and opaque cursor pagination. See [environment files](https://developers.openai.com/api/docs/guides/agents-api/environments/files).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **String** | The ID of the environment. | [required] |
**path** | Option<**String**> | Restrict the listing to this absolute workspace directory. |  |
**limit** | Option<**i64**> | The maximum number of files to return, between 1 and 100. |  |
**order** | Option<[**ListOrderParam**](.md)> | Sort by case-sensitive path components. Defaults to descending. |  |
**page** | Option<**String**> | The opaque token from the previous page. Keep the same path, order, and limit. |  |

### Return type

[**models::EnvironmentFileListResource**](EnvironmentFileListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_environment_templates

> models::EnvironmentTemplateListResource list_agent_environment_templates(limit, order, after)
List agent environment templates

Lists reusable environment templates without returning confidential values. See [reusing a hosted setup](https://developers.openai.com/api/docs/guides/agents-api/tools#reuse-a-hosted-plugin-setup).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20. |  |[default to 20]
**order** | Option<[**ListOrderParam**](.md)> | The order in which resources are returned. Defaults to `desc`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |

### Return type

[**models::EnvironmentTemplateListResource**](EnvironmentTemplateListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_artifacts

> models::SessionArtifactListResource list_agent_session_artifacts(session_id, order, environment_id, limit, after)
List agent session artifacts

Lists immutable artifacts published by completed hosted session turns. See [session artifacts](https://developers.openai.com/api/docs/guides/agents-api/environments/files#openai-hosted-artifacts).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**order** | Option<[**ListOrderParam**](.md)> | Sort by creation time and ID. Defaults to descending. |  |
**environment_id** | Option<**String**> | Restrict the listing to artifacts produced by this environment. |  |
**limit** | Option<**i64**> | The maximum number of artifacts to return, between 1 and 100. |  |
**after** | Option<**String**> | Return artifacts after this immutable artifact ID. |  |

### Return type

[**models::SessionArtifactListResource**](SessionArtifactListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_events

> models::SessionEvent list_agent_session_events(session_id)
Stream agent session events

Streams live events for an agent session. See [session events](https://developers.openai.com/api/docs/guides/agents-api/sessions/events).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |

### Return type

[**models::SessionEvent**](SessionEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/event-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_items

> models::SessionItemListResource list_agent_session_items(session_id, limit, order, after)
List agent session items

Lists items produced by the session's root agent, including its interactions with subagents. Each subagent has its own item history. See [inspecting agent output](https://developers.openai.com/api/docs/guides/agents-api/observability).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20. |  |[default to 20]
**order** | Option<[**ListOrderParam**](.md)> | The order in which resources are returned. Defaults to `desc`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |

### Return type

[**models::SessionItemListResource**](SessionItemListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_subagent_items

> models::SessionItemListResource list_agent_session_subagent_items(session_id, subagent_id, limit, order, after)
List subagent items

Lists this subagent's own items across all of its turns. See [subagent workflows](https://developers.openai.com/api/docs/guides/agents-api/multi-agent).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**subagent_id** | **String** | The ID of the subagent in this session. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20. |  |[default to 20]
**order** | Option<[**ListOrderParam**](.md)> | The order in which resources are returned. Defaults to `desc`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |

### Return type

[**models::SessionItemListResource**](SessionItemListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_subagent_turn_items

> models::SessionItemListResource list_agent_session_subagent_turn_items(session_id, subagent_id, turn_id, limit, order, after)
List subagent turn items

Lists items belonging to one turn of this subagent. See [subagent workflows](https://developers.openai.com/api/docs/guides/agents-api/multi-agent).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**subagent_id** | **String** | The ID of the subagent in this session. | [required] |
**turn_id** | **String** | The ID of a turn belonging to this subagent. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20. |  |[default to 20]
**order** | Option<[**ListOrderParam**](.md)> | The order in which resources are returned. Defaults to `desc`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |

### Return type

[**models::SessionItemListResource**](SessionItemListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_subagent_turns

> models::SessionTurnListResource list_agent_session_subagent_turns(session_id, subagent_id, limit, order, after)
List subagent turns

Lists all turns of this subagent, including turns after a resume. See [subagent workflows](https://developers.openai.com/api/docs/guides/agents-api/multi-agent).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**subagent_id** | **String** | The ID of the subagent in this session. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20. |  |[default to 20]
**order** | Option<[**ListOrderParam**](.md)> | The order in which resources are returned. Defaults to `desc`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |

### Return type

[**models::SessionTurnListResource**](SessionTurnListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_subagents

> models::ListAgentSessionSubagents200Response list_agent_session_subagents(session_id, limit, order, after)
List session subagents

Lists subagents in a session, including nested and closed subagents. See [subagent workflows](https://developers.openai.com/api/docs/guides/agents-api/multi-agent).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20. |  |[default to 20]
**order** | Option<[**ListOrderParam**](.md)> | The order in which resources are returned. Defaults to `desc`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |

### Return type

[**models::ListAgentSessionSubagents200Response**](listAgentSessionSubagents_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_turns

> models::SessionTurnListResource list_agent_session_turns(session_id, limit, order, after)
List agent session turns

Lists turns by creation time and turn ID. The after cursor is exclusive in the selected order. See [session turns](https://developers.openai.com/api/docs/guides/agents-api/sessions/manage#inspect-session-turns).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20. |  |[default to 20]
**order** | Option<[**ListOrderParam**](.md)> | The order in which resources are returned. Defaults to `desc`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |

### Return type

[**models::SessionTurnListResource**](SessionTurnListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_sessions

> models::SessionListResource list_agent_sessions(limit, order, agent_id, after)
List agent sessions

Lists managed agent sessions using ID-based pagination and the requested sort order. See [managing sessions](https://developers.openai.com/api/docs/guides/agents-api/sessions/manage).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The maximum number of resources to return. |  |
**order** | Option<[**ListOrderParam**](.md)> | Sort order by the `created_at` timestamp. Use `asc` for ascending order or `desc` for descending order. Defaults to `desc`. |  |
**agent_id** | Option<**String**> | Only return sessions whose root agent has this ID. Omit to return sessions for all agents. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |

### Return type

[**models::SessionListResource**](SessionListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agents

> models::AgentListResource list_agents(limit, order, after)
List agents

Lists reusable agents in the current project. See [agent configuration](https://developers.openai.com/api/docs/guides/agents-api/configuration).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The maximum number of resources to return. |  |
**order** | Option<[**ListOrderParam**](.md)> | The order in which resources are returned. Defaults to `desc`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |

### Return type

[**models::AgentListResource**](AgentListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent

> models::AgentResource retrieve_agent(agent_id)
Retrieve an agent

Retrieves a reusable agent by ID. See [agent configuration](https://developers.openai.com/api/docs/guides/agents-api/configuration).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** | The ID of the reusable agent. | [required] |

### Return type

[**models::AgentResource**](AgentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_environment

> models::PublicEnvironmentResource retrieve_agent_environment(environment_id)
Retrieve an agent environment

Retrieves an execution environment's connection status and safe installed metadata. See [environment lifecycle](https://developers.openai.com/api/docs/guides/agents-api/environments/lifecycle).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **String** | The ID of the environment. | [required] |

### Return type

[**models::PublicEnvironmentResource**](PublicEnvironmentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_environment_template

> models::EnvironmentTemplateResource retrieve_agent_environment_template(environment_template_id)
Retrieve an agent environment template

Retrieves reusable environment configuration without returning confidential values. See [reusing a hosted setup](https://developers.openai.com/api/docs/guides/agents-api/tools#reuse-a-hosted-plugin-setup).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_template_id** | **String** | The ID of the reusable environment template. | [required] |

### Return type

[**models::EnvironmentTemplateResource**](EnvironmentTemplateResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_session

> models::SessionResource retrieve_agent_session(session_id)
Retrieve an agent session

Retrieves the current state of a managed agent session. See [managing sessions](https://developers.openai.com/api/docs/guides/agents-api/sessions/manage).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |

### Return type

[**models::SessionResource**](SessionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_session_artifact

> models::SessionArtifactResource retrieve_agent_session_artifact(session_id, artifact_id)
Retrieve an agent session artifact

Retrieves immutable metadata for one durable session artifact. See [session artifacts](https://developers.openai.com/api/docs/guides/agents-api/environments/files#openai-hosted-artifacts).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session that owns the artifact. | [required] |
**artifact_id** | **String** | The immutable session artifact ID. | [required] |

### Return type

[**models::SessionArtifactResource**](SessionArtifactResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_session_artifact_content

> std::path::PathBuf retrieve_agent_session_artifact_content(session_id, artifact_id)
Retrieve agent session artifact content

Downloads immutable session artifact bytes after the execution environment expires. See [session artifacts](https://developers.openai.com/api/docs/guides/agents-api/environments/files#openai-hosted-artifacts).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session that owns the artifact. | [required] |
**artifact_id** | **String** | The immutable session artifact ID. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_session_subagent

> models::SubagentResource retrieve_agent_session_subagent(session_id, subagent_id)
Retrieve a session subagent

Retrieves a subagent belonging to this session. See [subagent workflows](https://developers.openai.com/api/docs/guides/agents-api/multi-agent).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**subagent_id** | **String** | The ID of the subagent in this session. | [required] |

### Return type

[**models::SubagentResource**](SubagentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_session_subagent_turn

> models::TurnResource retrieve_agent_session_subagent_turn(session_id, subagent_id, turn_id)
Retrieve a subagent turn

Retrieves a turn belonging to this subagent. See [subagent workflows](https://developers.openai.com/api/docs/guides/agents-api/multi-agent).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**subagent_id** | **String** | The ID of the subagent in this session. | [required] |
**turn_id** | **String** | The ID of a turn belonging to this subagent. | [required] |

### Return type

[**models::TurnResource**](TurnResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_session_turn

> models::TurnResource retrieve_agent_session_turn(session_id, turn_id)
Retrieve an agent session turn

Retrieves a turn's current status, timestamps, usage, and error. Returns 404 if the turn does not belong to the session. See [session turns](https://developers.openai.com/api/docs/guides/agents-api/sessions/manage#inspect-session-turns).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session that owns the turn. | [required] |
**turn_id** | **String** | The ID of the turn. | [required] |

### Return type

[**models::TurnResource**](TurnResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_agent

> models::AgentResource update_agent(agent_id, update_agent_params)
Update an agent

Updates a reusable agent. See [agent configuration](https://developers.openai.com/api/docs/guides/agents-api/configuration).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** | The ID of the reusable agent. | [required] |
**update_agent_params** | Option<[**UpdateAgentParams**](UpdateAgentParams.md)> |  |  |

### Return type

[**models::AgentResource**](AgentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_agent_environment_template

> models::EnvironmentTemplateResource update_agent_environment_template(environment_template_id, update_environment_template_params)
Update an agent environment template

Updates reusable environment configuration without returning confidential values. See [reusing a hosted setup](https://developers.openai.com/api/docs/guides/agents-api/tools#reuse-a-hosted-plugin-setup).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_template_id** | **String** | The ID of the reusable environment template. | [required] |
**update_environment_template_params** | Option<[**UpdateEnvironmentTemplateParams**](UpdateEnvironmentTemplateParams.md)> |  |  |

### Return type

[**models::EnvironmentTemplateResource**](EnvironmentTemplateResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_agent_session

> models::SessionResource update_agent_session(session_id, update_agent_session_params)
Update an agent session

Updates session metadata, model, reasoning effort, or service tier. Model settings apply to subsequent turns. Omitted fields are unchanged. See [managing sessions](https://developers.openai.com/api/docs/guides/agents-api/sessions/manage).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**update_agent_session_params** | Option<[**UpdateAgentSessionParams**](UpdateAgentSessionParams.md)> |  |  |

### Return type

[**models::SessionResource**](SessionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

