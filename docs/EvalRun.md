# EvalRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | The type of the object. Always \"eval.run\". | 
**id** | **String** | Unique identifier for the evaluation run. | 
**eval_id** | **String** | The identifier of the associated evaluation. | 
**status** | **String** | The status of the evaluation run. | 
**model** | **String** | The model that is evaluated, if applicable. | 
**name** | **String** | The name of the evaluation run. | 
**created_at** | **i32** | Unix timestamp (in seconds) when the evaluation run was created. | 
**report_url** | **String** | The URL to the rendered evaluation run report on the UI dashboard. | 
**result_counts** | [**models::EvalRunResultCounts**](EvalRun_result_counts.md) |  | 
**per_model_usage** | [**Vec<models::EvalRunPerModelUsageInner>**](EvalRun_per_model_usage_inner.md) | Usage statistics for each model during the evaluation run. | 
**per_testing_criteria_results** | [**Vec<models::EvalRunPerTestingCriteriaResultsInner>**](EvalRun_per_testing_criteria_results_inner.md) | Results per testing criteria applied during the evaluation run. | 
**data_source** | [**models::EvalRunDataSource**](EvalRun_data_source.md) |  | 
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.  Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | 
**error** | [**models::EvalApiError**](EvalApiError.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


