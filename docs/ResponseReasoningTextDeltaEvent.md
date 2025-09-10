# ResponseReasoningTextDeltaEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always `response.reasoning_text.delta`.  | 
**item_id** | **String** | The ID of the item this reasoning text delta is associated with.  | 
**output_index** | **i32** | The index of the output item this reasoning text delta is associated with.  | 
**content_index** | **i32** | The index of the reasoning content part this delta is associated with.  | 
**delta** | **String** | The text delta that was added to the reasoning content.  | 
**sequence_number** | **i32** | The sequence number of this event.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


