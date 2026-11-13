# \AgentsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_agent**](AgentsApi.md#create_agent) | **POST** /agents | Create an agent
[**create_agent_automation**](AgentsApi.md#create_agent_automation) | **POST** /agents/automations | Create an automation
[**create_agent_automation_invocation**](AgentsApi.md#create_agent_automation_invocation) | **POST** /agents/automations/{automation_id}/invocations | Invoke an automation
[**create_agent_environment**](AgentsApi.md#create_agent_environment) | **POST** /agents/environments | Create an agent environment
[**create_agent_environment_file**](AgentsApi.md#create_agent_environment_file) | **POST** /agents/environments/{environment_id}/files | Write an agent environment file
[**create_agent_environment_template**](AgentsApi.md#create_agent_environment_template) | **POST** /agents/environments/templates | Create an agent environment template
[**create_agent_session**](AgentsApi.md#create_agent_session) | **POST** /agents/sessions | Create an agent session
[**create_agent_session_events**](AgentsApi.md#create_agent_session_events) | **POST** /agents/sessions/{session_id}/events | Create agent session input events
[**create_agent_session_stream**](AgentsApi.md#create_agent_session_stream) | **POST** /agents/sessions | Create an agent session
[**delete_agent**](AgentsApi.md#delete_agent) | **DELETE** /agents/{agent_id} | Delete an agent
[**delete_agent_automation**](AgentsApi.md#delete_agent_automation) | **DELETE** /agents/automations/{automation_id} | Delete an automation
[**delete_agent_environment_template**](AgentsApi.md#delete_agent_environment_template) | **DELETE** /agents/environments/templates/{environment_template_id} | Delete an agent environment template
[**delete_agent_session**](AgentsApi.md#delete_agent_session) | **DELETE** /agents/sessions/{session_id} | Delete an agent session
[**delete_agent_session_artifact**](AgentsApi.md#delete_agent_session_artifact) | **DELETE** /agents/sessions/{session_id}/artifacts/{artifact_id} | Delete an agent session artifact
[**get_agent_automation**](AgentsApi.md#get_agent_automation) | **GET** /agents/automations/{automation_id} | Get an automation
[**get_agent_automation_invocation**](AgentsApi.md#get_agent_automation_invocation) | **GET** /agents/automations/{automation_id}/invocations/{invocation_id} | Get an automation invocation
[**list_agent_automation_invocations**](AgentsApi.md#list_agent_automation_invocations) | **GET** /agents/automations/{automation_id}/invocations | List automation invocations
[**list_agent_automations**](AgentsApi.md#list_agent_automations) | **GET** /agents/automations | List automations
[**list_agent_environment_files**](AgentsApi.md#list_agent_environment_files) | **GET** /agents/environments/{environment_id}/files | List agent environment files
[**list_agent_environment_templates**](AgentsApi.md#list_agent_environment_templates) | **GET** /agents/environments/templates | List agent environment templates
[**list_agent_environments**](AgentsApi.md#list_agent_environments) | **GET** /agents/environments | List agent environments
[**list_agent_session_artifacts**](AgentsApi.md#list_agent_session_artifacts) | **GET** /agents/sessions/{session_id}/artifacts | List agent session artifacts
[**list_agent_session_events**](AgentsApi.md#list_agent_session_events) | **GET** /agents/sessions/{session_id}/events | Stream agent session events
[**list_agent_session_items**](AgentsApi.md#list_agent_session_items) | **GET** /agents/sessions/{session_id}/items | List agent session items
[**list_agent_session_subagent_items**](AgentsApi.md#list_agent_session_subagent_items) | **GET** /agents/sessions/{session_id}/subagents/{subagent_id}/items | List subagent items
[**list_agent_session_subagent_turn_items**](AgentsApi.md#list_agent_session_subagent_turn_items) | **GET** /agents/sessions/{session_id}/subagents/{subagent_id}/turns/{turn_id}/items | List subagent turn items
[**list_agent_session_subagent_turns**](AgentsApi.md#list_agent_session_subagent_turns) | **GET** /agents/sessions/{session_id}/subagents/{subagent_id}/turns | List subagent turns
[**list_agent_session_subagents**](AgentsApi.md#list_agent_session_subagents) | **GET** /agents/sessions/{session_id}/subagents | List session subagents
[**list_agent_session_traces**](AgentsApi.md#list_agent_session_traces) | **GET** /agents/sessions/{session_id}/traces | List agent session traces
[**list_agent_session_turn_items**](AgentsApi.md#list_agent_session_turn_items) | **GET** /agents/sessions/{session_id}/turns/{turn_id}/items | List agent session turn items
[**list_agent_session_turns**](AgentsApi.md#list_agent_session_turns) | **GET** /agents/sessions/{session_id}/turns | List agent session turns
[**list_agent_sessions**](AgentsApi.md#list_agent_sessions) | **GET** /agents/sessions | List agent sessions
[**list_agents**](AgentsApi.md#list_agents) | **GET** /agents | List agents
[**pause_agent_automation**](AgentsApi.md#pause_agent_automation) | **POST** /agents/automations/{automation_id}/pause | Pause an automation
[**resume_agent_automation**](AgentsApi.md#resume_agent_automation) | **POST** /agents/automations/{automation_id}/resume | Resume an automation
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
[**update_agent_automation**](AgentsApi.md#update_agent_automation) | **POST** /agents/automations/{automation_id} | Update an automation
[**update_agent_environment_template**](AgentsApi.md#update_agent_environment_template) | **POST** /agents/environments/templates/{environment_template_id} | Update an agent environment template
[**update_agent_session**](AgentsApi.md#update_agent_session) | **POST** /agents/sessions/{session_id} | Update an agent session



## create_agent

> models::AgentResource create_agent(create_agent_params, options)
Create an agent

Creates a reusable agent without storing credentials. See [agent configuration](https://developers.openai.com/api/docs/guides/agents-api/configuration).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_agent_params** | Option<[**CreateAgentParams**](CreateAgentParams.md)> |  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_agent().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::AgentResource**](AgentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_automation

> models::AutomationResource create_agent_automation(create_automation_params, options)
Create an automation

Saves an active automation without invoking it immediately.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_automation_params** | [**CreateAutomationParams**](CreateAutomationParams.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_agent_automation().configuration(&configuration).create_automation_params(..).options(&options).call().await
```

### Return type

[**models::AutomationResource**](AutomationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_automation_invocation

> models::AutomationInvocationResource create_agent_automation_invocation(automation_id, options)
Invoke an automation

Queues a manual delivery without changing the saved schedule.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**automation_id** | **String** |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_agent_automation_invocation().configuration(&configuration).automation_id(..).options(&options).call().await
```

### Return type

[**models::AutomationInvocationResource**](AutomationInvocationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_environment

> models::PublicEnvironmentResource create_agent_environment(create_agent_environment_params, idempotency_key, options)
Create an agent environment

Creates an OpenAI-hosted environment before creating a session. Requires access to the prewarming beta.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_agent_environment_params** | [**CreateAgentEnvironmentParams**](CreateAgentEnvironmentParams.md) |  | [required] |
**idempotency_key** | Option<**String**> | Deduplicates creation for 24 hours within the authenticated organization, project, and creator. Retry the same JSON parameters with the same key to retrieve the original environment in its current state. Different parameters or an incomplete hosted creation return 409. Deleted environments are not recreated. After retention expires, the key may create a new environment. Without this header, each request creates a new environment. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_agent_environment().configuration(&configuration).create_agent_environment_params(..).options(&options).call().await
```

### Return type

[**models::PublicEnvironmentResource**](PublicEnvironmentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_environment_file

> models::EnvironmentFileResource create_agent_environment_file(environment_id, hosted_environment_file_param, options)
Write an agent environment file

Copies inline bytes or a Files API file into a connected execution environment, replacing an existing regular file at the same path. Upload between turns and wait for success before starting the next turn. See [environment files](https://developers.openai.com/api/docs/guides/agents-api/environments/files).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **String** | The ID of the environment. | [required] |
**hosted_environment_file_param** | Option<[**HostedEnvironmentFileParam**](HostedEnvironmentFileParam.md)> |  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_agent_environment_file().configuration(&configuration).environment_id(..).options(&options).call().await
```

### Return type

[**models::EnvironmentFileResource**](EnvironmentFileResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_environment_template

> models::EnvironmentTemplateResource create_agent_environment_template(create_environment_template_params, options)
Create an agent environment template

Creates reusable environment configuration without returning confidential setup commands or environment values. See [reusing a hosted setup](https://developers.openai.com/api/docs/guides/agents-api/tools#reuse-a-hosted-plugin-setup).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_environment_template_params** | Option<[**CreateEnvironmentTemplateParams**](CreateEnvironmentTemplateParams.md)> |  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_agent_environment_template().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::EnvironmentTemplateResource**](EnvironmentTemplateResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_session

> models::SessionResource create_agent_session(create_agent_session_params, options)
Create an agent session

Creates a managed agent session, optionally submits initial input, and returns the session or streams its events when stream is true. See [running sessions](https://developers.openai.com/api/docs/guides/agents-api/sessions).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_agent_session_params** | Option<[**CreateAgentSessionParams**](CreateAgentSessionParams.md)> |  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_agent_session().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::SessionResource**](SessionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_session_events

> create_agent_session_events(session_id, idempotency_key, create_session_events_params, options)
Create agent session input events

Submits message, cancellation, tool-result, or computer-use approval-response events to a managed agent session. Cancellation can recover a still-open turn whose backend execution has ended by marking it cancelled and abandoning unpublished outputs. Saved results, published files, and existing terminal outcomes are preserved. HTTP 202 confirms acceptance, not durable completion. See [session events](https://developers.openai.com/api/docs/guides/agents-api/sessions/events).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**idempotency_key** | Option<**String**> | An optional client-generated key that makes retries of submitted messages idempotent. |  |
**create_session_events_params** | Option<[**CreateSessionEventsParams**](CreateSessionEventsParams.md)> |  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_agent_session_events().configuration(&configuration).session_id(..).options(&options).call().await
```

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_session_stream

> models::SessionEvent create_agent_session_stream(create_agent_session_params, options)
Create an agent session

Creates a managed agent session, optionally submits initial input, and returns the session or streams its events when stream is true. See [running sessions](https://developers.openai.com/api/docs/guides/agents-api/sessions).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_agent_session_params** | Option<[**CreateAgentSessionParams**](CreateAgentSessionParams.md)> |  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_agent_session_stream().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::SessionEvent**](SessionEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/event-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_agent

> models::DeletedAgentResource delete_agent(agent_id, options)
Delete an agent

Deletes a reusable agent. See [agent configuration](https://developers.openai.com/api/docs/guides/agents-api/configuration).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** | The ID of the reusable agent. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
delete_agent().configuration(&configuration).agent_id(..).options(&options).call().await
```

### Return type

[**models::DeletedAgentResource**](DeletedAgentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_agent_automation

> models::DeletedAutomationResource delete_agent_automation(automation_id, options)
Delete an automation

Deletes the automation and prevents future invocations without deleting its target session.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**automation_id** | **String** |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
delete_agent_automation().configuration(&configuration).automation_id(..).options(&options).call().await
```

### Return type

[**models::DeletedAutomationResource**](DeletedAutomationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_agent_environment_template

> models::DeletedEnvironmentTemplateResource delete_agent_environment_template(environment_template_id, options)
Delete an agent environment template

Deletes reusable environment configuration and all confidential template inputs. See [reusing a hosted setup](https://developers.openai.com/api/docs/guides/agents-api/tools#reuse-a-hosted-plugin-setup).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_template_id** | **String** | The ID of the reusable environment template. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
delete_agent_environment_template().configuration(&configuration).environment_template_id(..).options(&options).call().await
```

### Return type

[**models::DeletedEnvironmentTemplateResource**](DeletedEnvironmentTemplateResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_agent_session

> models::DeletedSessionResource delete_agent_session(session_id, options)
Delete an agent session

Removes a managed agent session from the public API and returns a deletion confirmation. If backend execution has ended, deletion can cancel a still-open public turn and abandon unpublished outputs. Running execution must be cancelled first. Physical cleanup may continue asynchronously. See [managing sessions](https://developers.openai.com/api/docs/guides/agents-api/sessions/manage).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
delete_agent_session().configuration(&configuration).session_id(..).options(&options).call().await
```

### Return type

[**models::DeletedSessionResource**](DeletedSessionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_agent_session_artifact

> models::DeletedSessionArtifactResource delete_agent_session_artifact(session_id, artifact_id, options)
Delete an agent session artifact

Deletes an immutable session artifact without deleting its live environment file or original Files API object. See [session artifacts](https://developers.openai.com/api/docs/guides/agents-api/environments/files#openai-hosted-artifacts).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session that owns the artifact. | [required] |
**artifact_id** | **String** | The immutable session artifact ID. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
delete_agent_session_artifact().configuration(&configuration).session_id(..).artifact_id(..).options(&options).call().await
```

### Return type

[**models::DeletedSessionArtifactResource**](DeletedSessionArtifactResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_agent_automation

> models::AutomationResource get_agent_automation(automation_id, options)
Get an automation

Retrieves a project-owned automation.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**automation_id** | **String** |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_agent_automation().configuration(&configuration).automation_id(..).options(&options).call().await
```

### Return type

[**models::AutomationResource**](AutomationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_agent_automation_invocation

> models::AutomationInvocationResource get_agent_automation_invocation(automation_id, invocation_id, options)
Get an automation invocation

Retrieves a delivery record for a project-owned automation.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**automation_id** | **String** |  | [required] |
**invocation_id** | **String** |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_agent_automation_invocation().configuration(&configuration).automation_id(..).invocation_id(..).options(&options).call().await
```

### Return type

[**models::AutomationInvocationResource**](AutomationInvocationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_automation_invocations

> models::ListAgentAutomationInvocations200Response list_agent_automation_invocations(automation_id, limit, order, after, options)
List automation invocations

Lists delivery records for a project-owned automation.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**automation_id** | **String** |  | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_automation_invocations().configuration(&configuration).automation_id(..).options(&options).call().await
```

### Return type

[**models::ListAgentAutomationInvocations200Response**](ListAgentAutomationInvocations200Response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_automations

> models::ListAgentAutomations200Response list_agent_automations(limit, order, after, session_id, options)
List automations

Lists project-owned automations, optionally filtered by target session.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**session_id** | Option<**String**> | Matches existing-session targets, including agent-created automations. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_automations().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::ListAgentAutomations200Response**](ListAgentAutomations200Response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_environment_files

> models::EnvironmentFileListResource list_agent_environment_files(environment_id, path, limit, order, page, options)
List agent environment files

Lists live files on a connected execution environment with optional directory filtering and opaque cursor pagination. See [environment files](https://developers.openai.com/api/docs/guides/agents-api/environments/files).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **String** | The ID of the environment. | [required] |
**path** | Option<**String**> | Restrict the listing to this absolute workspace directory. |  |
**limit** | Option<**i64**> | The maximum number of files to return, between 1 and 100. |  |
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | Sort by case-sensitive path components. Defaults to descending. |  |
**page** | Option<**String**> | The opaque token from the previous page. Keep the same path, order, and limit. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_environment_files().configuration(&configuration).environment_id(..).options(&options).call().await
```

### Return type

[**models::EnvironmentFileListResource**](EnvironmentFileListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_environment_templates

> models::EnvironmentTemplateListResource list_agent_environment_templates(limit, order, after, options)
List agent environment templates

Lists reusable environment templates without returning confidential values. See [reusing a hosted setup](https://developers.openai.com/api/docs/guides/agents-api/tools#reuse-a-hosted-plugin-setup).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_environment_templates().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::EnvironmentTemplateListResource**](EnvironmentTemplateListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_environments

> models::AgentEnvironmentListResource list_agent_environments(limit, order, after, r#type, options)
List agent environments

Lists OpenAI-hosted environments owned by the authenticated principal. Requires access to the prewarming beta.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**r#type** | Option<[**EnvironmentTypeParam**](EnvironmentTypeParam.md)> | The hosting type to list. Defaults to `openai_hosted`. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_environments().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::AgentEnvironmentListResource**](AgentEnvironmentListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_artifacts

> models::SessionArtifactListResource list_agent_session_artifacts(session_id, limit, order, after, environment_id, options)
List agent session artifacts

Lists immutable artifacts published by completed hosted session turns. See [session artifacts](https://developers.openai.com/api/docs/guides/agents-api/environments/files#openai-hosted-artifacts).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**environment_id** | Option<**String**> | Restrict the listing to artifacts produced by this environment. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_session_artifacts().configuration(&configuration).session_id(..).options(&options).call().await
```

### Return type

[**models::SessionArtifactListResource**](SessionArtifactListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_events

> models::SessionEvent list_agent_session_events(session_id, include_obfuscation, options)
Stream agent session events

Streams live events for an agent session. See [session events](https://developers.openai.com/api/docs/guides/agents-api/sessions/events).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**include_obfuscation** | Option<**bool**> | Whether to include obfuscation padding in events on this connection. Defaults to `true`. |  |[default to true]
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_session_events().configuration(&configuration).session_id(..).options(&options).call().await
```

### Return type

[**models::SessionEvent**](SessionEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/event-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_items

> models::SessionItemListResource list_agent_session_items(session_id, limit, order, after, options)
List agent session items

Lists items produced by the session's root agent, including its interactions with subagents. Each subagent has its own item history. See [inspecting agent output](https://developers.openai.com/api/docs/guides/agents-api/observability).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_session_items().configuration(&configuration).session_id(..).options(&options).call().await
```

### Return type

[**models::SessionItemListResource**](SessionItemListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_subagent_items

> models::SessionItemListResource list_agent_session_subagent_items(session_id, subagent_id, limit, order, after, options)
List subagent items

Lists this subagent's own items across all of its turns. See [subagent workflows](https://developers.openai.com/api/docs/guides/agents-api/multi-agent).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**subagent_id** | **String** | The ID of the subagent in this session. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_session_subagent_items().configuration(&configuration).session_id(..).subagent_id(..).options(&options).call().await
```

### Return type

[**models::SessionItemListResource**](SessionItemListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_subagent_turn_items

> models::SessionItemListResource list_agent_session_subagent_turn_items(session_id, subagent_id, turn_id, limit, order, after, options)
List subagent turn items

Lists items belonging to one turn of this subagent. See [subagent workflows](https://developers.openai.com/api/docs/guides/agents-api/multi-agent).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**subagent_id** | **String** | The ID of the subagent in this session. | [required] |
**turn_id** | **String** | The ID of a turn belonging to this subagent. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_session_subagent_turn_items().configuration(&configuration).session_id(..).subagent_id(..).turn_id(..).options(&options).call().await
```

### Return type

[**models::SessionItemListResource**](SessionItemListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_subagent_turns

> models::SessionTurnListResource list_agent_session_subagent_turns(session_id, subagent_id, limit, order, after, options)
List subagent turns

Lists all turns of this subagent, including turns after a resume. See [subagent workflows](https://developers.openai.com/api/docs/guides/agents-api/multi-agent).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**subagent_id** | **String** | The ID of the subagent in this session. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_session_subagent_turns().configuration(&configuration).session_id(..).subagent_id(..).options(&options).call().await
```

### Return type

[**models::SessionTurnListResource**](SessionTurnListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_subagents

> models::ListAgentSessionSubagents200Response list_agent_session_subagents(session_id, limit, order, after, options)
List session subagents

Lists subagents in a session, including nested and closed subagents. See [subagent workflows](https://developers.openai.com/api/docs/guides/agents-api/multi-agent).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_session_subagents().configuration(&configuration).session_id(..).options(&options).call().await
```

### Return type

[**models::ListAgentSessionSubagents200Response**](ListAgentSessionSubagents200Response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_traces

> models::SessionTraceListResource list_agent_session_traces(session_id, limit, order, after, options)
List agent session traces

Lists published root-turn traces as OTLP JSON, ordered by turn creation time and ID. Unpublished traces are skipped. Each page returns data available when read; it does not wait for late traces. Trace reads and the JSON response are limited to 16 MiB per request. If the limit is exceeded, request fewer traces.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_session_traces().configuration(&configuration).session_id(..).options(&options).call().await
```

### Return type

[**models::SessionTraceListResource**](SessionTraceListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_turn_items

> models::SessionItemListResource list_agent_session_turn_items(session_id, turn_id, limit, order, after, options)
List agent session turn items

Lists items belonging to one root-agent turn, including its interactions with subagents. See [inspecting agent output](https://developers.openai.com/api/docs/guides/agents-api/observability).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session that owns the turn. | [required] |
**turn_id** | **String** | The ID of the turn. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_session_turn_items().configuration(&configuration).session_id(..).turn_id(..).options(&options).call().await
```

### Return type

[**models::SessionItemListResource**](SessionItemListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_session_turns

> models::SessionTurnListResource list_agent_session_turns(session_id, limit, order, after, options)
List agent session turns

Lists turns by creation time and turn ID. The after cursor is exclusive in the selected order. See [session turns](https://developers.openai.com/api/docs/guides/agents-api/sessions/manage#inspect-session-turns).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_session_turns().configuration(&configuration).session_id(..).options(&options).call().await
```

### Return type

[**models::SessionTurnListResource**](SessionTurnListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agent_sessions

> models::SessionListResource list_agent_sessions(limit, order, after, agent_id, metadata, options)
List agent sessions

Lists managed agent sessions using ID-based pagination and the requested sort order. See [managing sessions](https://developers.openai.com/api/docs/guides/agents-api/sessions/manage).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**agent_id** | Option<**String**> | Only return sessions whose root agent has this ID. Omit to return sessions for all agents. |  |
**metadata** | Option<**std::collections::HashMap<String, String>**> | Exact string matches supplied as `metadata[key]=value`. All supplied pairs must match. Up to 16 pairs, with keys from 1 to 64 characters and values up to 512 characters. New sessions and metadata updates may take time to appear in filtered results. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agent_sessions().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::SessionListResource**](SessionListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agents

> models::AgentListResource list_agents(limit, order, after, options)
List agents

Lists reusable agents in the current project. See [agent configuration](https://developers.openai.com/api/docs/guides/agents-api/configuration).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The maximum number of resources to return, between 1 and 100. Defaults to 20.  Defaults to `20`. |  |[default to 20]
**order** | Option<[**ListOrderParam**](ListOrderParam.md)> | The order in which resources are returned. Defaults to `desc`.  Defaults to `\"desc\"`. |  |
**after** | Option<**String**> | Return resources after this resource ID in the selected order. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_agents().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::AgentListResource**](AgentListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause_agent_automation

> models::AutomationResource pause_agent_automation(automation_id, options)
Pause an automation

Pauses future scheduled delivery without stopping an active session.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**automation_id** | **String** |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
pause_agent_automation().configuration(&configuration).automation_id(..).options(&options).call().await
```

### Return type

[**models::AutomationResource**](AutomationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_agent_automation

> models::AutomationResource resume_agent_automation(automation_id, options)
Resume an automation

Resumes scheduled delivery; one-time automations without a pending invocation require a future run_at.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**automation_id** | **String** |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
resume_agent_automation().configuration(&configuration).automation_id(..).options(&options).call().await
```

### Return type

[**models::AutomationResource**](AutomationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent

> models::AgentResource retrieve_agent(agent_id, options)
Retrieve an agent

Retrieves a reusable agent by ID. See [agent configuration](https://developers.openai.com/api/docs/guides/agents-api/configuration).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** | The ID of the reusable agent. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_agent().configuration(&configuration).agent_id(..).options(&options).call().await
```

### Return type

[**models::AgentResource**](AgentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_environment

> models::PublicEnvironmentResource retrieve_agent_environment(environment_id, options)
Retrieve an agent environment

Retrieves an execution environment's connection status and safe installed metadata. See [environment lifecycle](https://developers.openai.com/api/docs/guides/agents-api/environments/lifecycle).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **String** | The ID of the environment. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_agent_environment().configuration(&configuration).environment_id(..).options(&options).call().await
```

### Return type

[**models::PublicEnvironmentResource**](PublicEnvironmentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_environment_template

> models::EnvironmentTemplateResource retrieve_agent_environment_template(environment_template_id, options)
Retrieve an agent environment template

Retrieves reusable environment configuration without returning confidential values. See [reusing a hosted setup](https://developers.openai.com/api/docs/guides/agents-api/tools#reuse-a-hosted-plugin-setup).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_template_id** | **String** | The ID of the reusable environment template. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_agent_environment_template().configuration(&configuration).environment_template_id(..).options(&options).call().await
```

### Return type

[**models::EnvironmentTemplateResource**](EnvironmentTemplateResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_session

> models::SessionResource retrieve_agent_session(session_id, options)
Retrieve an agent session

Retrieves the current state of a managed agent session. See [managing sessions](https://developers.openai.com/api/docs/guides/agents-api/sessions/manage).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_agent_session().configuration(&configuration).session_id(..).options(&options).call().await
```

### Return type

[**models::SessionResource**](SessionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_session_artifact

> models::SessionArtifactResource retrieve_agent_session_artifact(session_id, artifact_id, options)
Retrieve an agent session artifact

Retrieves immutable metadata for one durable session artifact. See [session artifacts](https://developers.openai.com/api/docs/guides/agents-api/environments/files#openai-hosted-artifacts).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session that owns the artifact. | [required] |
**artifact_id** | **String** | The immutable session artifact ID. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_agent_session_artifact().configuration(&configuration).session_id(..).artifact_id(..).options(&options).call().await
```

### Return type

[**models::SessionArtifactResource**](SessionArtifactResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_session_artifact_content

> std::path::PathBuf retrieve_agent_session_artifact_content(session_id, artifact_id, options)
Retrieve agent session artifact content

Downloads immutable session artifact bytes after the execution environment expires. See [session artifacts](https://developers.openai.com/api/docs/guides/agents-api/environments/files#openai-hosted-artifacts).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session that owns the artifact. | [required] |
**artifact_id** | **String** | The immutable session artifact ID. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_agent_session_artifact_content().configuration(&configuration).session_id(..).artifact_id(..).options(&options).call().await
```

### Return type

**std::path::PathBuf**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_session_subagent

> models::SubagentResource retrieve_agent_session_subagent(session_id, subagent_id, options)
Retrieve a session subagent

Retrieves a subagent belonging to this session. See [subagent workflows](https://developers.openai.com/api/docs/guides/agents-api/multi-agent).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**subagent_id** | **String** | The ID of the subagent in this session. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_agent_session_subagent().configuration(&configuration).session_id(..).subagent_id(..).options(&options).call().await
```

### Return type

[**models::SubagentResource**](SubagentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_session_subagent_turn

> models::TurnResource retrieve_agent_session_subagent_turn(session_id, subagent_id, turn_id, options)
Retrieve a subagent turn

Retrieves a turn belonging to this subagent. See [subagent workflows](https://developers.openai.com/api/docs/guides/agents-api/multi-agent).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**subagent_id** | **String** | The ID of the subagent in this session. | [required] |
**turn_id** | **String** | The ID of a turn belonging to this subagent. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_agent_session_subagent_turn().configuration(&configuration).session_id(..).subagent_id(..).turn_id(..).options(&options).call().await
```

### Return type

[**models::TurnResource**](TurnResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_agent_session_turn

> models::TurnResource retrieve_agent_session_turn(session_id, turn_id, options)
Retrieve an agent session turn

Retrieves a turn's current status, timestamps, usage, and error. Returns 404 if the turn does not belong to the session. See [session turns](https://developers.openai.com/api/docs/guides/agents-api/sessions/manage#inspect-session-turns).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session that owns the turn. | [required] |
**turn_id** | **String** | The ID of the turn. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
retrieve_agent_session_turn().configuration(&configuration).session_id(..).turn_id(..).options(&options).call().await
```

### Return type

[**models::TurnResource**](TurnResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_agent

> models::AgentResource update_agent(agent_id, update_agent_params, options)
Update an agent

Updates a reusable agent. See [agent configuration](https://developers.openai.com/api/docs/guides/agents-api/configuration).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** | The ID of the reusable agent. | [required] |
**update_agent_params** | Option<[**UpdateAgentParams**](UpdateAgentParams.md)> |  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
update_agent().configuration(&configuration).agent_id(..).options(&options).call().await
```

### Return type

[**models::AgentResource**](AgentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_agent_automation

> models::AutomationResource update_agent_automation(automation_id, update_automation_params, options)
Update an automation

Updates supplied fields; target and metadata replace the whole field.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**automation_id** | **String** |  | [required] |
**update_automation_params** | [**UpdateAutomationParams**](UpdateAutomationParams.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
update_agent_automation().configuration(&configuration).automation_id(..).update_automation_params(..).options(&options).call().await
```

### Return type

[**models::AutomationResource**](AutomationResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_agent_environment_template

> models::EnvironmentTemplateResource update_agent_environment_template(environment_template_id, update_environment_template_params, options)
Update an agent environment template

Updates reusable environment configuration without returning confidential values. See [reusing a hosted setup](https://developers.openai.com/api/docs/guides/agents-api/tools#reuse-a-hosted-plugin-setup).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_template_id** | **String** | The ID of the reusable environment template. | [required] |
**update_environment_template_params** | Option<[**UpdateEnvironmentTemplateParams**](UpdateEnvironmentTemplateParams.md)> |  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
update_agent_environment_template().configuration(&configuration).environment_template_id(..).options(&options).call().await
```

### Return type

[**models::EnvironmentTemplateResource**](EnvironmentTemplateResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_agent_session

> models::SessionResource update_agent_session(session_id, update_agent_session_params, options)
Update an agent session

Updates session tools, metadata, model, reasoning effort, service tier, or the default prompt cache lifetime. Model settings apply to subsequent turns. Omitted fields are unchanged. See [managing sessions](https://developers.openai.com/api/docs/guides/agents-api/sessions/manage).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session. | [required] |
**update_agent_session_params** | Option<[**UpdateAgentSessionParams**](UpdateAgentSessionParams.md)> |  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
update_agent_session().configuration(&configuration).session_id(..).options(&options).call().await
```

### Return type

[**models::SessionResource**](SessionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

