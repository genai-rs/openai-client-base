# BetaResponseOutputItemAddedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> | The agent that owns this multi-agent streaming event. | [optional]
**r#type** | **Type** | The type of the event. Always `response.output_item.added`.  (enum: response.output_item.added) | 
**output_index** | **i32** | The index of the output item that was added.  | 
**sequence_number** | **i32** | The sequence number of this event.  | 
**item** | [**models::BetaOutputItem**](BetaOutputItem.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


