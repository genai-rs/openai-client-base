# EvalGraderTextSimilarity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of grader. (enum: text_similarity) | 
**name** | **String** | The name of the grader. | 
**input** | **String** | The text being graded. | 
**reference** | **String** | The text being graded against. | 
**evaluation_metric** | **EvaluationMetric** | The evaluation metric to use. One of `cosine`, `fuzzy_match`, `bleu`,  `gleu`, `meteor`, `rouge_1`, `rouge_2`, `rouge_3`, `rouge_4`, `rouge_5`,  or `rouge_l`.  (enum: cosine, fuzzy_match, bleu, gleu, meteor, rouge_1, rouge_2, rouge_3, rouge_4, rouge_5, rouge_l) | 
**pass_threshold** | **f64** | The threshold for the score. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


