# CreateEvalCustomDataSourceConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of data source. Always `custom`. | 
**item_schema** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | The json schema for each row in the data source. | 
**include_sample_schema** | Option<**bool**> | Whether the eval should expect you to populate the sample namespace (ie, by generating responses off of your data source) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


