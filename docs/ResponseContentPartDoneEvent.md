# ResponseContentPartDoneEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always `response.content_part.done`.  | 
**item_id** | **String** | The ID of the output item that the content part was added to.  | 
**output_index** | **i32** | The index of the output item that the content part was added to.  | 
**content_index** | **i32** | The index of the content part that is done.  | 
**sequence_number** | **i32** | The sequence number of this event. | 
**part** | [**models::OutputContent**](OutputContent.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


