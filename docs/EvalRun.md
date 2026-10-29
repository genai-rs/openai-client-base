# EvalRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | The type of the object. Always \"eval.run\". (enum: eval.run) | 
**id** | **String** | Unique identifier for the evaluation run. | 
**eval_id** | **String** | The identifier of the associated evaluation. | 
**status** | **String** | The status of the evaluation run. | 
**model** | Option<**String**> | The model that is evaluated, if applicable. | 
**name** | Option<**String**> | The name of the evaluation run. | 
**created_at** | **i32** | Unix timestamp (in seconds) when the evaluation run was created. | 
**report_url** | **String** | The URL to the rendered evaluation run report on the UI dashboard. | 
**result_counts** | [**models::EvalRunResultCounts**](EvalRunResultCounts.md) |  | 
**per_model_usage** | Option<[**Vec<models::EvalRunPerModelUsageInner>**](EvalRunPerModelUsageInner.md)> | Usage statistics for each model during the evaluation run. | 
**per_testing_criteria_results** | Option<[**Vec<models::EvalRunPerTestingCriteriaResultsInner>**](EvalRunPerTestingCriteriaResultsInner.md)> | Results per testing criteria applied during the evaluation run. | 
**data_source** | [**models::EvalRunDataSource**](EvalRunDataSource.md) |  | 
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.  Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | 
**error** | Option<[**models::EvalApiError**](EvalApiError.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


