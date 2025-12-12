# FineTuneReinforcementMethodGrader

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The object type, which is always `string_check`. | 
**name** | **String** | The name of the grader. | 
**input** | [**Vec<models::EvalItem>**](EvalItem.md) | The input messages evaluated by the grader. Supports text, output text, input image, and input audio content blocks, and may include template strings.  | 
**reference** | **String** | The text being graded against. | 
**operation** | **String** | The string check operation to perform. One of `eq`, `ne`, `like`, or `ilike`. | 
**evaluation_metric** | **String** | The evaluation metric to use. One of `cosine`, `fuzzy_match`, `bleu`,  `gleu`, `meteor`, `rouge_1`, `rouge_2`, `rouge_3`, `rouge_4`, `rouge_5`,  or `rouge_l`.  | 
**source** | **String** | The source code of the python script. | 
**image_tag** | Option<**String**> | The image tag to use for the python script. | [optional]
**model** | **String** | The model to use for the evaluation. | 
**sampling_params** | Option<[**models::GraderScoreModelSamplingParams**](GraderScoreModel_sampling_params.md)> |  | [optional]
**range** | Option<**Vec<f64>**> | The range of the score. Defaults to `[0, 1]`. | [optional]
**graders** | [**models::GraderMultiGraders**](GraderMulti_graders.md) |  | 
**calculate_output** | **String** | A formula to calculate the output based on grader results. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


