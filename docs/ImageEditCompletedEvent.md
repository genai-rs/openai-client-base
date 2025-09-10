# ImageEditCompletedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always `image_edit.completed`.  | 
**b64_json** | **String** | Base64-encoded final edited image data, suitable for rendering as an image.  | 
**created_at** | **i32** | The Unix timestamp when the event was created.  | 
**size** | **String** | The size of the edited image.  | 
**quality** | **String** | The quality setting for the edited image.  | 
**background** | **String** | The background setting for the edited image.  | 
**output_format** | **String** | The output format for the edited image.  | 
**usage** | [**models::ImagesUsage**](ImagesUsage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


