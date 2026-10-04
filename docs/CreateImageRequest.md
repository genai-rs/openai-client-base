# CreateImageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prompt** | **String** | A text description of the desired image(s). The maximum length is 32000 characters. | 
**model** | **String** |  | 
**n** | Option<**i32**> | The number of images to generate. Must be between 1 and 10. | [optional]
**quality** | Option<**String**> |  | [optional]
**response_format** | Option<**String**> | Legacy response format parameter for retired image models. Unsupported for GPT image models, which always return base64-encoded images. | [optional]
**output_format** | Option<**String**> |  | [optional]
**output_compression** | Option<**i32**> | The compression level (0-100%) for the generated images. This parameter is only supported for the GPT image models with the `webp` or `jpeg` output formats, and defaults to 100. | [optional]
**stream** | Option<**bool**> | Generate the image in streaming mode. Defaults to `false`. See the [Image generation guide](https://developers.openai.com/api/docs/guides/image-generation) for more information. This parameter is only supported for the GPT image models.  | [optional]
**partial_images** | Option<**i32**> | The number of partial images to generate. This parameter is used for streaming responses that return partial images. Value must be between 0 and 3. When set to 0, the response will be a single image sent in one streaming event.  Note that the final image may be sent before the full number of partial images are generated if the full image is generated more quickly.  | [optional]
**size** | Option<[**models::CreateImageRequestSize**](CreateImageRequest_size.md)> |  | [optional]
**moderation** | Option<**String**> |  | [optional]
**background** | Option<**String**> |  | [optional]
**style** | Option<**String**> | Legacy style parameter for retired image models. Unsupported for GPT image models; describe the desired style in the prompt instead. | [optional]
**user** | Option<**String**> | A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


