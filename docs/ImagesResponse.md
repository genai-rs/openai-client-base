# ImagesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | **i32** | The Unix timestamp (in seconds) of when the image was created. | 
**data** | Option<[**Vec<models::Image>**](Image.md)> | The list of generated images. | [optional]
**background** | Option<**String**> | The background parameter used for the image generation. Either `transparent` or `opaque`. | [optional]
**output_format** | Option<**String**> | The output format of the image generation. Either `png`, `webp`, or `jpeg`. | [optional]
**size** | Option<**String**> | The size of the image generated. Either `1024x1024`, `1024x1536`, or `1536x1024`. | [optional]
**quality** | Option<**String**> | The quality of the image generated. Either `low`, `medium`, or `high`. | [optional]
**usage** | Option<[**models::ImageGenUsage**](ImageGenUsage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


