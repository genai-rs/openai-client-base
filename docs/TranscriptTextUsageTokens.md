# TranscriptTextUsageTokens

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the usage object. Always `tokens` for this variant. | 
**input_tokens** | **i32** | Number of input tokens billed for this request. | 
**input_token_details** | Option<[**models::TranscriptTextUsageTokensInputTokenDetails**](TranscriptTextUsageTokens_input_token_details.md)> |  | [optional]
**output_tokens** | **i32** | Number of output tokens generated. | 
**total_tokens** | **i32** | Total number of tokens used (input + output). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


