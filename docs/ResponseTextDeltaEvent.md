# ResponseTextDeltaEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always `response.output_text.delta`.  | 
**item_id** | **String** | The ID of the output item that the text delta was added to.  | 
**output_index** | **i32** | The index of the output item that the text delta was added to.  | 
**content_index** | **i32** | The index of the content part that the text delta was added to.  | 
**delta** | **String** | The text delta that was added.  | 
**sequence_number** | **i32** | The sequence number for this event. | 
**logprobs** | [**Vec<models::ResponseLogProb>**](ResponseLogProb.md) | The log probabilities of the tokens in the delta.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


