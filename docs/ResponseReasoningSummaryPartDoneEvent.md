# ResponseReasoningSummaryPartDoneEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always `response.reasoning_summary_part.done`.  | 
**item_id** | **String** | The ID of the item this summary part is associated with.  | 
**output_index** | **i32** | The index of the output item this summary part is associated with.  | 
**summary_index** | **i32** | The index of the summary part within the reasoning summary.  | 
**sequence_number** | **i32** | The sequence number of this event.  | 
**part** | [**models::ResponseReasoningSummaryPartDoneEventPart**](ResponseReasoningSummaryPartDoneEvent_part.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


