# ResponseFunctionCallArgumentsWsDone

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**item_id** | **String** | The ID of the item. | 
**output_index** | **i32** | The index of the output item. | 
**sequence_number** | **i32** | The sequence number of this event. | 
**arguments** | **String** | The function-call arguments. | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the originating `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


