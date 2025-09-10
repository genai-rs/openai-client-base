# EvalRunOutputItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | The type of the object. Always \"eval.run.output_item\". | 
**id** | **String** | Unique identifier for the evaluation run output item. | 
**run_id** | **String** | The identifier of the evaluation run associated with this output item. | 
**eval_id** | **String** | The identifier of the evaluation group. | 
**created_at** | **i32** | Unix timestamp (in seconds) when the evaluation run was created. | 
**status** | **String** | The status of the evaluation run. | 
**datasource_item_id** | **i32** | The identifier for the data source item. | 
**datasource_item** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Details of the input data source item. | 
**results** | [**Vec<std::collections::HashMap<String, serde_json::Value>>**](std::collections::HashMap.md) | A list of results from the evaluation run. | 
**sample** | [**models::EvalRunOutputItemSample**](EvalRunOutputItem_sample.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


