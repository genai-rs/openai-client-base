# CreateEvalRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the evaluation. | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.   Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | [optional]
**data_source_config** | [**models::CreateEvalRequestDataSourceConfig**](CreateEvalRequest_data_source_config.md) |  | 
**testing_criteria** | [**Vec<models::CreateEvalRequestTestingCriteriaInner>**](CreateEvalRequest_testing_criteria_inner.md) | A list of graders for all eval runs in this group. Graders can reference variables in the data source using double curly braces notation, like `{{item.variable_name}}`. To reference the model's output, use the `sample` namespace (ie, `{{sample.output_text}}`). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


