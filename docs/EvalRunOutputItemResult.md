# EvalRunOutputItemResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the grader. | 
**r#type** | Option<**String**> | The grader type (for example, \"string-check-grader\"). | [optional]
**score** | **f64** | The numeric score produced by the grader. | 
**passed** | **bool** | Whether the grader considered the output a pass. | 
**sample** | Option<[**models::EvalRunOutputItemResultSample**](EvalRunOutputItemResult_sample.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


