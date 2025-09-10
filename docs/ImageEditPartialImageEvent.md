# ImageEditPartialImageEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always `image_edit.partial_image`.  | 
**b64_json** | **String** | Base64-encoded partial image data, suitable for rendering as an image.  | 
**created_at** | **i32** | The Unix timestamp when the event was created.  | 
**size** | **String** | The size of the requested edited image.  | 
**quality** | **String** | The quality setting for the requested edited image.  | 
**background** | **String** | The background setting for the requested edited image.  | 
**output_format** | **String** | The output format for the requested edited image.  | 
**partial_image_index** | **i32** | 0-based index for the partial image (streaming).  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


