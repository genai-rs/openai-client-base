# \DecisionsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_decision**](DecisionsApi.md#create_decision) | **POST** /decisions | Create a decision



## create_decision

> models::DecisionResponse create_decision(decision_request)
Create a decision

Evaluate ordered classification and scoring questions against shared input. Answers are returned in question order.  Supply input as a string or user messages containing text and inline images. Only user messages with `input_text` and `input_image` parts are supported; non-user roles, function calls, files, audio, and item references are not supported. Images require a data URL, not an external URL or file ID. At most 128 images are allowed across the request.  Each question can return a refusal instead of a scored answer. A refusal has type `refusal` and the corresponding question name, or null if unnamed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**decision_request** | [**DecisionRequest**](DecisionRequest.md) |  | [required] |

### Return type

[**models::DecisionResponse**](DecisionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

