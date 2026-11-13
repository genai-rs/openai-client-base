# \EvalsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_eval_run**](EvalsApi.md#cancel_eval_run) | **POST** /evals/{eval_id}/runs/{run_id}/cancel | Cancel eval run
[**create_eval**](EvalsApi.md#create_eval) | **POST** /evals | Create eval
[**create_eval_run**](EvalsApi.md#create_eval_run) | **POST** /evals/{eval_id}/runs | Create eval run
[**delete_eval**](EvalsApi.md#delete_eval) | **DELETE** /evals/{eval_id} | Delete an eval
[**delete_eval_run**](EvalsApi.md#delete_eval_run) | **DELETE** /evals/{eval_id}/runs/{run_id} | Delete eval run
[**get_eval**](EvalsApi.md#get_eval) | **GET** /evals/{eval_id} | Get an eval
[**get_eval_run**](EvalsApi.md#get_eval_run) | **GET** /evals/{eval_id}/runs/{run_id} | Get an eval run
[**get_eval_run_output_item**](EvalsApi.md#get_eval_run_output_item) | **GET** /evals/{eval_id}/runs/{run_id}/output_items/{output_item_id} | Get an output item of an eval run
[**get_eval_run_output_items**](EvalsApi.md#get_eval_run_output_items) | **GET** /evals/{eval_id}/runs/{run_id}/output_items | Get eval run output items
[**get_eval_runs**](EvalsApi.md#get_eval_runs) | **GET** /evals/{eval_id}/runs | Get eval runs
[**list_evals**](EvalsApi.md#list_evals) | **GET** /evals | List evals
[**update_eval**](EvalsApi.md#update_eval) | **POST** /evals/{eval_id} | Update an eval



## cancel_eval_run

> models::EvalRun cancel_eval_run(eval_id, run_id, options)
Cancel eval run

Cancel an ongoing evaluation run. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eval_id** | **String** | The ID of the evaluation whose run you want to cancel. | [required] |
**run_id** | **String** | The ID of the run to cancel. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
cancel_eval_run().configuration(&configuration).eval_id(..).run_id(..).options(&options).call().await
```

### Return type

[**models::EvalRun**](EvalRun.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_eval

> models::Eval create_eval(create_eval_request, options)
Create eval

Create the structure of an evaluation that can be used to test a model's performance. An evaluation is a set of testing criteria and the config for a data source, which dictates the schema of the data used in the evaluation. After creating an evaluation, you can run it on different models and model parameters. We support several types of graders and datasources. For more information, see the [Evals guide](https://developers.openai.com/api/docs/guides/evals). 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_eval_request** | [**CreateEvalRequest**](CreateEvalRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_eval().configuration(&configuration).create_eval_request(..).options(&options).call().await
```

### Return type

[**models::Eval**](Eval.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_eval_run

> models::EvalRun create_eval_run(eval_id, create_eval_run_request, options)
Create eval run

Kicks off a new run for a given evaluation, specifying the data source, and what model configuration to use to test. The datasource will be validated against the schema specified in the config of the evaluation. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eval_id** | **String** | The ID of the evaluation to create a run for. | [required] |
**create_eval_run_request** | [**CreateEvalRunRequest**](CreateEvalRunRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_eval_run().configuration(&configuration).eval_id(..).create_eval_run_request(..).options(&options).call().await
```

### Return type

[**models::EvalRun**](EvalRun.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_eval

> models::DeleteEval200Response delete_eval(eval_id, options)
Delete an eval

Delete an evaluation. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eval_id** | **String** | The ID of the evaluation to delete. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
delete_eval().configuration(&configuration).eval_id(..).options(&options).call().await
```

### Return type

[**models::DeleteEval200Response**](DeleteEval200Response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_eval_run

> models::DeleteEvalRun200Response delete_eval_run(eval_id, run_id, options)
Delete eval run

Delete an eval run. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eval_id** | **String** | The ID of the evaluation to delete the run from. | [required] |
**run_id** | **String** | The ID of the run to delete. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
delete_eval_run().configuration(&configuration).eval_id(..).run_id(..).options(&options).call().await
```

### Return type

[**models::DeleteEvalRun200Response**](DeleteEvalRun200Response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eval

> models::Eval get_eval(eval_id, options)
Get an eval

Get an evaluation by ID. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eval_id** | **String** | The ID of the evaluation to retrieve. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_eval().configuration(&configuration).eval_id(..).options(&options).call().await
```

### Return type

[**models::Eval**](Eval.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eval_run

> models::EvalRun get_eval_run(eval_id, run_id, options)
Get an eval run

Get an evaluation run by ID. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eval_id** | **String** | The ID of the evaluation to retrieve runs for. | [required] |
**run_id** | **String** | The ID of the run to retrieve. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_eval_run().configuration(&configuration).eval_id(..).run_id(..).options(&options).call().await
```

### Return type

[**models::EvalRun**](EvalRun.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eval_run_output_item

> models::EvalRunOutputItem get_eval_run_output_item(eval_id, run_id, output_item_id, options)
Get an output item of an eval run

Get an evaluation run output item by ID. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eval_id** | **String** | The ID of the evaluation to retrieve runs for. | [required] |
**run_id** | **String** | The ID of the run to retrieve. | [required] |
**output_item_id** | **String** | The ID of the output item to retrieve. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_eval_run_output_item().configuration(&configuration).eval_id(..).run_id(..).output_item_id(..).options(&options).call().await
```

### Return type

[**models::EvalRunOutputItem**](EvalRunOutputItem.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eval_run_output_items

> models::EvalRunOutputItemList get_eval_run_output_items(eval_id, run_id, after, limit, status, order, options)
Get eval run output items

Get a list of output items for an evaluation run. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eval_id** | **String** | The ID of the evaluation to retrieve runs for. | [required] |
**run_id** | **String** | The ID of the run to retrieve output items for. | [required] |
**after** | Option<**String**> | Identifier for the last output item from the previous pagination request. |  |
**limit** | Option<**i64**> | Number of output items to retrieve.  Defaults to `20`. |  |[default to 20]
**status** | Option<**String**> | Filter output items by status. Use `failed` to filter by failed output items or `pass` to filter by passed output items.  |  |
**order** | Option<**String**> | Sort order for output items by timestamp. Use `asc` for ascending order or `desc` for descending order. Defaults to `asc`.  Defaults to `\"asc\"`. |  |[default to asc]
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_eval_run_output_items().configuration(&configuration).eval_id(..).run_id(..).options(&options).call().await
```

### Return type

[**models::EvalRunOutputItemList**](EvalRunOutputItemList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eval_runs

> models::EvalRunList get_eval_runs(eval_id, after, limit, order, status, options)
Get eval runs

Get a list of runs for an evaluation. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eval_id** | **String** | The ID of the evaluation to retrieve runs for. | [required] |
**after** | Option<**String**> | Identifier for the last run from the previous pagination request. |  |
**limit** | Option<**i64**> | Number of runs to retrieve.  Defaults to `20`. |  |[default to 20]
**order** | Option<**String**> | Sort order for runs by timestamp. Use `asc` for ascending order or `desc` for descending order. Defaults to `asc`.  Defaults to `\"asc\"`. |  |[default to asc]
**status** | Option<**String**> | Filter runs by status. One of `queued` | `in_progress` | `failed` | `completed` | `canceled`. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_eval_runs().configuration(&configuration).eval_id(..).options(&options).call().await
```

### Return type

[**models::EvalRunList**](EvalRunList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_evals

> models::EvalList list_evals(after, limit, order, order_by, options)
List evals

List evaluations for a project. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**String**> | Identifier for the last eval from the previous pagination request. |  |
**limit** | Option<**i64**> | Number of evals to retrieve.  Defaults to `20`. |  |[default to 20]
**order** | Option<**String**> | Sort order for evals by timestamp. Use `asc` for ascending order or `desc` for descending order.  Defaults to `\"asc\"`. |  |[default to asc]
**order_by** | Option<**String**> | Evals can be ordered by creation time or last updated time. Use `created_at` for creation time or `updated_at` for last updated time.  Defaults to `\"created_at\"`. |  |[default to created_at]
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_evals().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::EvalList**](EvalList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_eval

> models::Eval update_eval(eval_id, update_eval_request, options)
Update an eval

Update certain properties of an evaluation. 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eval_id** | **String** | The ID of the evaluation to update. | [required] |
**update_eval_request** | [**UpdateEvalRequest**](UpdateEvalRequest.md) | Request to update an evaluation | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
update_eval().configuration(&configuration).eval_id(..).update_eval_request(..).options(&options).call().await
```

### Return type

[**models::Eval**](Eval.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

