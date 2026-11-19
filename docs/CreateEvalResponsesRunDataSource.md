# CreateEvalResponsesRunDataSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of run data source. Always `responses`.  Defaults to `\"responses\"`. (enum: responses) | 
**input_messages** | Option<[**models::CreateEvalResponsesRunDataSourceInputMessages**](CreateEvalResponsesRunDataSourceInputMessages.md)> |  | [optional]
**sampling_params** | Option<[**models::CreateEvalResponsesRunDataSourceSamplingParams**](CreateEvalResponsesRunDataSourceSamplingParams.md)> |  | [optional]
**model** | Option<**String**> | The model to use to sample outputs with the Responses API. | [optional]
**source** | [**models::CreateEvalResponsesRunDataSourceSource**](CreateEvalResponsesRunDataSourceSource.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


