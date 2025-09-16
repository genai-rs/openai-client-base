# \ImagesApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_image**](ImagesApi.md#create_image) | **POST** /images/generations | Create image
[**create_image_edit**](ImagesApi.md#create_image_edit) | **POST** /images/edits | Create image edit
[**create_image_variation**](ImagesApi.md#create_image_variation) | **POST** /images/variations | Create image variation



## create_image

> models::ImagesResponse create_image(create_image_request)
Create image

Creates an image given a prompt. [Learn more](https://platform.openai.com/docs/guides/images). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_image_request** | [**CreateImageRequest**](CreateImageRequest.md) |  | [required] |

### Return type

[**models::ImagesResponse**](ImagesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_image_edit

> models::ImagesResponse create_image_edit(image, prompt, mask, background, model, n, size, response_format, output_format, output_compression, user, input_fidelity, stream, partial_images, quality)
Create image edit

Creates an edited or extended image given one or more source images and a prompt. This endpoint only supports `gpt-image-1` and `dall-e-2`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image** | **String** |  | [required] |
**prompt** | **String** | A text description of the desired image(s). The maximum length is 1000 characters for `dall-e-2`, and 32000 characters for `gpt-image-1`. | [required] |
**mask** | Option<**std::path::PathBuf**> | An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where `image` should be edited. If there are multiple images provided, the mask will be applied on the first image. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`. |  |
**background** | Option<**String**> | Allows to set transparency for the background of the generated image(s).  This parameter is only supported for `gpt-image-1`. Must be one of  `transparent`, `opaque` or `auto` (default value). When `auto` is used, the  model will automatically determine the best background for the image.  If `transparent`, the output format needs to support transparency, so it  should be set to either `png` (default value) or `webp`.  |  |
**model** | Option<**String**> |  |  |
**n** | Option<**i32**> | The number of images to generate. Must be between 1 and 10. |  |
**size** | Option<**String**> | The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, and one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`. |  |
**response_format** | Option<**String**> | The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This parameter is only supported for `dall-e-2`, as `gpt-image-1` will always return base64-encoded images. |  |
**output_format** | Option<**String**> | The format in which the generated images are returned. This parameter is only supported for `gpt-image-1`. Must be one of `png`, `jpeg`, or `webp`. The default value is `png`.  |  |
**output_compression** | Option<**i32**> | The compression level (0-100%) for the generated images. This parameter  is only supported for `gpt-image-1` with the `webp` or `jpeg` output  formats, and defaults to 100.  |  |
**user** | Option<**String**> | A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).  |  |
**input_fidelity** | Option<[**models::ImageInputFidelity**](ImageInputFidelity.md)> |  |  |
**stream** | Option<**bool**> | Edit the image in streaming mode. Defaults to `false`. See the  [Image generation guide](https://platform.openai.com/docs/guides/image-generation) for more information.  |  |
**partial_images** | Option<**i32**> | The number of partial images to generate. This parameter is used for streaming responses that return partial images. Value must be between 0 and 3. When set to 0, the response will be a single image sent in one streaming event.  Note that the final image may be sent before the full number of partial images are generated if the full image is generated more quickly.  |  |
**quality** | Option<**String**> | The quality of the image that will be generated. `high`, `medium` and `low` are only supported for `gpt-image-1`. `dall-e-2` only supports `standard` quality. Defaults to `auto`.  |  |

### Return type

[**models::ImagesResponse**](ImagesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_image_variation

> models::ImagesResponse create_image_variation(image, model, n, response_format, size, user)
Create image variation

Creates a variation of a given image. This endpoint only supports `dall-e-2`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image** | **std::path::PathBuf** | The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and square. | [required] |
**model** | Option<**String**> |  |  |
**n** | Option<**i32**> | The number of images to generate. Must be between 1 and 10. |  |
**response_format** | Option<**String**> | The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. |  |
**size** | Option<**String**> | The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`. |  |
**user** | Option<**String**> | A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).  |  |

### Return type

[**models::ImagesResponse**](ImagesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

