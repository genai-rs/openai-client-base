# Eval

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | The object type. | 
**id** | **String** | Unique identifier for the evaluation. | 
**name** | **String** | The name of the evaluation. | 
**data_source_config** | [**models::EvalDataSourceConfig**](Eval_data_source_config.md) |  | 
**testing_criteria** | [**Vec<models::EvalTestingCriteriaInner>**](Eval_testing_criteria_inner.md) | A list of testing criteria. | 
**created_at** | **i32** | The Unix timestamp (in seconds) for when the eval was created. | 
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.  Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


