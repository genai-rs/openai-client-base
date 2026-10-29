# CreateEvalResponsesRunDataSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of run data source. Always `responses`. (enum: responses) | 
**input_messages** | Option<[**models::CreateEvalResponsesRunDataSourceInputMessages**](CreateEvalResponsesRunDataSourceInputMessages.md)> |  | [optional]
**sampling_params** | Option<[**models::CreateEvalResponsesRunDataSourceSamplingParams**](CreateEvalResponsesRunDataSourceSamplingParams.md)> |  | [optional]
**model** | Option<**String**> | The name of the model to use for generating completions (e.g. \"o3-mini\"). | [optional]
**source** | [**models::CreateEvalResponsesRunDataSourceSource**](CreateEvalResponsesRunDataSourceSource.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


