# GraderScoreModelSamplingParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seed** | Option<**i32**> | A seed value to initialize the randomness, during sampling.  | [optional]
**top_p** | Option<**f64**> | An alternative to temperature for nucleus sampling; 1.0 includes all tokens.  | [optional]
**temperature** | Option<**f64**> | A higher temperature increases randomness in the outputs.  | [optional]
**max_completions_tokens** | Option<**i32**> | The maximum number of tokens the grader model may generate in its response.  | [optional]
**reasoning_effort** | Option<[**models::ReasoningEffort**](ReasoningEffort.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


