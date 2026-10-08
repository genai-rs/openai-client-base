# \DecisionsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_decision**](DecisionsApi.md#create_decision) | **POST** /decisions | Create a decision



## create_decision

> models::DecisionResponse create_decision(decision_request)
Create a decision

Use this endpoint to ask classification or scoring questions about the same input. You’ll get the answers back in the order you asked the questions.  For text, you can pass a string. You can also send user messages containing `input_text` and `input_image` parts, with up to 128 images per request. Images must be data URLs; external URLs and file IDs aren’t accepted. Other message roles, function calls, files, audio, and item references aren’t supported.  Sometimes a question returns a refusal instead of an answer. The result has type `refusal` and includes the question’s name, or `null` if you didn’t give it one.

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

