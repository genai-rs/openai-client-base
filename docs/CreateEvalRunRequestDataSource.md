# CreateEvalRunRequestDataSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of data source. Always `jsonl`. | 
**source** | [**models::CreateEvalResponsesRunDataSourceSource**](CreateEvalResponsesRunDataSource_source.md) |  | 
**input_messages** | Option<[**models::CreateEvalResponsesRunDataSourceInputMessages**](CreateEvalResponsesRunDataSource_input_messages.md)> |  | [optional]
**sampling_params** | Option<[**models::CreateEvalResponsesRunDataSourceSamplingParams**](CreateEvalResponsesRunDataSource_sampling_params.md)> |  | [optional]
**model** | Option<**String**> | The name of the model to use for generating completions (e.g. \"o3-mini\"). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


