# ResponseImageGenCallPartialImageEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always 'response.image_generation_call.partial_image'. | 
**output_index** | **i32** | The index of the output item in the response's output array. | 
**item_id** | **String** | The unique identifier of the image generation item being processed. | 
**sequence_number** | **i32** | The sequence number of the image generation item being processed. | 
**partial_image_index** | **i32** | 0-based index for the partial image (backend is 1-based, but this is 0-based for the user). | 
**partial_image_b64** | **String** | Base64-encoded partial image data, suitable for rendering as an image. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


