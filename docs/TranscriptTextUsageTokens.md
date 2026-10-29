# TranscriptTextUsageTokens

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the usage object. Always `tokens` for this variant. (enum: tokens) | 
**input_tokens** | **i32** | Number of input tokens billed for this request. | 
**input_token_details** | Option<[**models::TranscriptTextUsageTokensInputTokenDetails**](TranscriptTextUsageTokensInputTokenDetails.md)> |  | [optional]
**output_tokens** | **i32** | Number of output tokens generated. | 
**total_tokens** | **i32** | Total number of tokens used (input + output). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


