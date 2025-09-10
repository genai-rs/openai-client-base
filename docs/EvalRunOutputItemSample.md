# EvalRunOutputItemSample

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**input** | [**Vec<models::EvalRunOutputItemSampleInputInner>**](EvalRunOutputItem_sample_input_inner.md) | An array of input messages. | 
**output** | [**Vec<models::EvalRunOutputItemSampleOutputInner>**](EvalRunOutputItem_sample_output_inner.md) | An array of output messages. | 
**finish_reason** | **String** | The reason why the sample generation was finished. | 
**model** | **String** | The model used for generating the sample. | 
**usage** | [**models::EvalRunOutputItemSampleUsage**](EvalRunOutputItem_sample_usage.md) |  | 
**error** | [**models::EvalApiError**](EvalApiError.md) |  | 
**temperature** | **f64** | The sampling temperature used. | 
**max_completion_tokens** | **i32** | The maximum number of tokens allowed for completion. | 
**top_p** | **f64** | The top_p value used for sampling. | 
**seed** | **i32** | The seed used for generating the sample. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


