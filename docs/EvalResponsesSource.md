# EvalResponsesSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of run data source. Always `responses`. | 
**metadata** | Option<[**serde_json::Value**](.md)> | Metadata filter for the responses. This is a query parameter used to select responses. | [optional]
**model** | Option<**String**> | The name of the model to find responses for. This is a query parameter used to select responses. | [optional]
**instructions_search** | Option<**String**> | Optional string to search the 'instructions' field. This is a query parameter used to select responses. | [optional]
**created_after** | Option<**i32**> | Only include items created after this timestamp (inclusive). This is a query parameter used to select responses. | [optional]
**created_before** | Option<**i32**> | Only include items created before this timestamp (inclusive). This is a query parameter used to select responses. | [optional]
**reasoning_effort** | Option<[**models::ReasoningEffort**](ReasoningEffort.md)> |  | [optional]
**temperature** | Option<**f64**> | Sampling temperature. This is a query parameter used to select responses. | [optional]
**top_p** | Option<**f64**> | Nucleus sampling parameter. This is a query parameter used to select responses. | [optional]
**users** | Option<**Vec<String>**> | List of user identifiers. This is a query parameter used to select responses. | [optional]
**tools** | Option<**Vec<String>**> | List of tool names. This is a query parameter used to select responses. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


