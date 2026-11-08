# CompletionUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completion_tokens** | **i64** | Number of tokens in the generated completion.  Defaults to `0`. | 
**prompt_tokens** | **i64** | Number of tokens in the prompt.  Defaults to `0`. | 
**total_tokens** | **i64** | Total number of tokens used in the request (prompt + completion).  Defaults to `0`. | 
**completion_tokens_details** | Option<[**models::CompletionUsageCompletionTokensDetails**](CompletionUsageCompletionTokensDetails.md)> |  | [optional]
**prompt_tokens_details** | Option<[**models::CompletionUsagePromptTokensDetails**](CompletionUsagePromptTokensDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


