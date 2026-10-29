# ImagesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**safety_identifier** | Option<**String**> | The effective safety identifier used for safety monitoring and abuse detection. This is the non-empty `safety_identifier` from the request, or the deprecated `user` value when `safety_identifier` is omitted, null, or empty. | [optional]
**created** | **i32** | The Unix timestamp (in seconds) of when the image was created. | 
**data** | Option<[**Vec<models::Image>**](Image.md)> | The list of generated images. | [optional]
**background** | Option<**Background**> | The background parameter used for the image generation. Either `transparent` or `opaque`. (enum: transparent, opaque) | [optional]
**output_format** | Option<**OutputFormat**> | The output format of the image generation. Either `png`, `webp`, or `jpeg`. (enum: png, webp, jpeg) | [optional]
**size** | Option<[**models::ImagesResponseSize**](ImagesResponseSize.md)> |  | [optional]
**quality** | Option<**Quality**> | The quality of the image generated. One of `low`, `medium`, `high`, `xhigh`, or `max`. (enum: low, medium, high, xhigh, max) | [optional]
**usage** | Option<[**models::ImageGenUsage**](ImageGenUsage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


