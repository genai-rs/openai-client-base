# CompactResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for the compacted response. | 
**object** | **String** | The object type. Always `response.compaction`. | 
**output** | [**Vec<serde_json::Value>**](serde_json::Value.md) | The compacted list of output items. This is a list of all user messages, followed by a single compaction item. | 
**created_at** | **i32** | Unix timestamp (in seconds) when the compacted conversation was created. | 
**usage** | [**models::ResponseUsage**](ResponseUsage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


