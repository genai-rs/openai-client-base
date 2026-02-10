# CreateEvalResponsesRunDataSourceSamplingParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reasoning_effort** | Option<[**models::ReasoningEffort**](ReasoningEffort.md)> |  | [optional]
**temperature** | Option<**f64**> | A higher temperature increases randomness in the outputs. | [optional]
**max_completion_tokens** | Option<**i32**> | The maximum number of tokens in the generated output. | [optional]
**top_p** | Option<**f64**> | An alternative to temperature for nucleus sampling; 1.0 includes all tokens. | [optional]
**seed** | Option<**i32**> | A seed value to initialize the randomness, during sampling. | [optional]
**tools** | Option<[**Vec<models::Tool>**](Tool.md)> | An array of tools the model may call while generating a response. You can specify which tool to use by setting the `tool_choice` parameter.  The two categories of tools you can provide the model are:  - **Built-in tools**: Tools that are provided by OpenAI that extend the   model's capabilities, like [web search](/docs/guides/tools-web-search)   or [file search](/docs/guides/tools-file-search). Learn more about   [built-in tools](/docs/guides/tools). - **Function calls (custom tools)**: Functions that are defined by you,   enabling the model to call your own code. Learn more about   [function calling](/docs/guides/function-calling).  | [optional]
**text** | Option<[**models::CreateEvalResponsesRunDataSourceSamplingParamsText**](CreateEvalResponsesRunDataSource_sampling_params_text.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


