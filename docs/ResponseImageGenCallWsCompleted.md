# ResponseImageGenCallWsCompleted

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always 'response.image_generation_call.completed'. | 
**output_index** | **i32** | The index of the output item in the response's output array. | 
**sequence_number** | **i32** | The sequence number of this event. | 
**item_id** | **String** | The unique identifier of the image generation item being processed. | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the originating `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


