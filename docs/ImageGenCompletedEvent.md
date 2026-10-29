# ImageGenCompletedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**safety_identifier** | Option<**String**> | The effective safety identifier used for safety monitoring and abuse detection. This is the non-empty `safety_identifier` from the request, or the deprecated `user` value when `safety_identifier` is omitted, null, or empty. | [optional]
**r#type** | **Type** | The type of the event. Always `image_generation.completed`.  (enum: image_generation.completed) | 
**b64_json** | **String** | Base64-encoded image data, suitable for rendering as an image.  | 
**created_at** | **i32** | The Unix timestamp when the event was created.  | 
**size** | [**models::ImageEditCompletedEventSize**](ImageEditCompletedEventSize.md) |  | 
**quality** | **Quality** | The quality setting for the generated image.  (enum: low, medium, high, xhigh, max, auto) | 
**background** | **Background** | The background setting for the generated image.  (enum: transparent, opaque, auto) | 
**output_format** | **OutputFormat** | The output format for the generated image.  (enum: png, webp, jpeg) | 
**usage** | [**models::ImagesUsage**](ImagesUsage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


