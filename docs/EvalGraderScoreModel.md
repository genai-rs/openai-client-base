# EvalGraderScoreModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The object type, which is always `score_model`. | 
**name** | **String** | The name of the grader. | 
**model** | **String** | ID of the model to use | 
**sampling_params** | Option<[**models::GraderScoreModelSamplingParams**](GraderScoreModel_sampling_params.md)> |  | [optional]
**input** | [**Vec<models::EvalItem>**](EvalItem.md) | The input text. This may include template strings. | 
**range** | Option<**Vec<f64>**> | The range of the score. Defaults to `[0, 1]`. | [optional]
**pass_threshold** | Option<**f64**> | The threshold for the score. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


