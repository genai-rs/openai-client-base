# RunGraderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**grader** | [**models::FineTuneReinforcementMethodGrader**](FineTuneReinforcementMethod_grader.md) |  | 
**item** | Option<[**serde_json::Value**](.md)> | The dataset item provided to the grader. This will be used to populate  the `item` namespace. See [the guide](/docs/guides/graders) for more details.   | [optional]
**model_sample** | **String** | The model sample to be evaluated. This value will be used to populate  the `sample` namespace. See [the guide](/docs/guides/graders) for more details. The `output_json` variable will be populated if the model sample is a  valid JSON string.    | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


