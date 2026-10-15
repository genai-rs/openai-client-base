# \DecisionsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_decision**](DecisionsApi.md#create_decision) | **POST** /decisions | Create a decision



## create_decision

> models::DecisionResponse create_decision(decision_request)
Create a decision

Use this endpoint to quickly answer classification or scoring questions about some input. You can ask multiple questions in one request.

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

