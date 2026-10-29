# GraderLabelModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The object type, which is always `label_model`. (enum: label_model) | 
**name** | **String** | The name of the grader. | 
**model** | **String** | The model to use for the evaluation. Must support structured outputs. | 
**input** | [**Vec<models::EvalItem>**](EvalItem.md) |  | 
**labels** | **Vec<String>** | The labels to assign to each item in the evaluation. | 
**passing_labels** | **Vec<String>** | The labels that indicate a passing result. Must be a subset of labels. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


