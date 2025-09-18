# EvalTestingCriteriaInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The object type, which is always `label_model`. | 
**name** | **String** | The name of the grader. | 
**model** | **String** | ID of the model to use | 
**input** | [**Vec<models::EvalItem>**](EvalItem.md) | The input text. This may include template strings. | 
**labels** | **Vec<String>** | The labels to assign to each item in the evaluation. | 
**passing_labels** | **Vec<String>** | The labels that indicate a passing result. Must be a subset of labels. | 
**reference** | **String** | The text being graded against. | 
**operation** | **String** | The string check operation to perform. One of `eq`, `ne`, `like`, or `ilike`. | 
**evaluation_metric** | **String** | The evaluation metric to use. One of `cosine`, `fuzzy_match`, `bleu`,  `gleu`, `meteor`, `rouge_1`, `rouge_2`, `rouge_3`, `rouge_4`, `rouge_5`,  or `rouge_l`.  | 
**pass_threshold** | **f64** | The threshold for the score. | 
**source** | **String** | The source code of the python script. | 
**image_tag** | Option<**String**> | The image tag to use for the python script. | [optional]
**sampling_params** | Option<[**models::GraderScoreModelSamplingParams**](GraderScoreModel_sampling_params.md)> |  | [optional]
**range** | Option<**Vec<f64>**> | The range of the score. Defaults to `[0, 1]`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


