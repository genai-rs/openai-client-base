# CreateEvalCompletionsRunDataSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of run data source. Always `completions`. (enum: completions) | 
**input_messages** | Option<[**models::CreateEvalCompletionsRunDataSourceInputMessages**](CreateEvalCompletionsRunDataSourceInputMessages.md)> |  | [optional]
**sampling_params** | Option<[**models::CreateEvalCompletionsRunDataSourceSamplingParams**](CreateEvalCompletionsRunDataSourceSamplingParams.md)> |  | [optional]
**model** | Option<**String**> | The name of the model to use for generating completions (e.g. \"o3-mini\"). | [optional]
**source** | [**models::CreateEvalCompletionsRunDataSourceSource**](CreateEvalCompletionsRunDataSourceSource.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


