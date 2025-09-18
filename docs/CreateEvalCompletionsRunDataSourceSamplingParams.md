# CreateEvalCompletionsRunDataSourceSamplingParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reasoning_effort** | Option<[**models::ReasoningEffort**](ReasoningEffort.md)> |  | [optional]
**temperature** | Option<**f64**> | A higher temperature increases randomness in the outputs. | [optional]
**max_completion_tokens** | Option<**i32**> | The maximum number of tokens in the generated output. | [optional]
**top_p** | Option<**f64**> | An alternative to temperature for nucleus sampling; 1.0 includes all tokens. | [optional]
**seed** | Option<**i32**> | A seed value to initialize the randomness, during sampling. | [optional]
**response_format** | Option<[**models::CreateChatCompletionRequestAllOfResponseFormat**](CreateChatCompletionRequest_allOf_response_format.md)> |  | [optional]
**tools** | Option<[**Vec<models::ChatCompletionTool>**](ChatCompletionTool.md)> | A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


