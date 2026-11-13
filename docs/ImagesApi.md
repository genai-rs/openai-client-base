# \ImagesApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_image**](ImagesApi.md#create_image) | **POST** /images/generations | Create image
[**create_image_edit**](ImagesApi.md#create_image_edit) | **POST** /images/edits | Create image edit
[**create_image_edit_json**](ImagesApi.md#create_image_edit_json) | **POST** /images/edits | Create image edit
[**create_image_edit_json_stream**](ImagesApi.md#create_image_edit_json_stream) | **POST** /images/edits | Create image edit
[**create_image_edit_stream**](ImagesApi.md#create_image_edit_stream) | **POST** /images/edits | Create image edit
[**create_image_stream**](ImagesApi.md#create_image_stream) | **POST** /images/generations | Create image
[**create_image_variation**](ImagesApi.md#create_image_variation) | **POST** /images/variations | Create image variation



## create_image

> models::ImagesResponse create_image(create_image_request, options)
Create image

Creates an image given a prompt using a GPT Image model. [Learn more](https://developers.openai.com/api/docs/guides/images-vision). 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_image_request** | [**CreateImageRequest**](CreateImageRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_image().configuration(&configuration).create_image_request(..).options(&options).call().await
```

### Return type

[**models::ImagesResponse**](ImagesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_image_edit

> models::ImagesResponse create_image_edit(image, prompt, model, frame_count, frame_duration_ms, complexity, mask, markup, background, n, size, response_format, output_format, svg, output_compression, safety_identifier, user, input_fidelity, stream, partial_images, quality, options)
Create image edit

Creates an edited or extended image given one or more source images and a prompt. This endpoint supports GPT Image models.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image** | **timvw_rust_runtime::Upload** |  | [required] |
**prompt** | **String** | A text description of the desired image(s). The maximum length is 32000 characters for the GPT image models. | [required] |
**model** | **String** |  | [required] |
**frame_count** | Option<**i64**> | Number of frames for GIF output. Accepts integers from 2 through 16. Defaults to 9 when omitted. Only supported with output_format=gif. The requested size must satisfy the selected model's GIF size limits. GIF output does not support streaming, partial images, masks, or custom output compression. Requires access to the GIF preview for the selected model. |  |
**frame_duration_ms** | Option<**i64**> | Requested delay per GIF frame, in milliseconds. Defaults to 200 when omitted. GIF output rounds to the nearest 10 ms and caps the encoded delay at 655,350 ms per frame. Only supported with output_format=gif. |  |
**complexity** | Option<[**models::Complexity**](Complexity.md)> |  |  |
**mask** | Option<**timvw_rust_runtime::Upload**> | An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where `image` should be edited. If there are multiple images provided, the mask will be applied on the first image. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`. |  |
**markup** | Option<**timvw_rust_runtime::Upload**> | A transparent PNG overlaid on the first image to guide the edit. Must match that image's dimensions after orientation and contain transparent pixels. Supports GPT Image 2.5 and 2.6. Cannot be combined with mask. |  |
**background** | Option<**String**> | Set the background of the generated image(s). This parameter is only supported for the GPT image models. Must be one of `transparent`, `opaque`, or `auto` (default value). When `auto` is used, the model will automatically determine the best background for the image.  `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08` snapshots, support `opaque` and `transparent` backgrounds. Transparent backgrounds are available for supported GPT Image models. For `gpt-image-2` and `gpt-image-2-2026-04-21`, this support is in preview. When using `transparent`, set the output format to `png` or `webp`.  Defaults to `\\\"auto\\\"`. |  |
**n** | Option<**i64**> | The number of images to generate. Must be between 1 and 10.  Defaults to `1`. |  |
**size** | Option<[**models::CreateImageEditRequestSize**](CreateImageEditRequestSize.md)> |  |  |
**response_format** | Option<**String**> | Legacy response format parameter for retired image models. Unsupported for GPT image models, which always return base64-encoded images. |  |
**output_format** | Option<[**models::ImagesApiOutputFormat**](ImagesApiOutputFormat.md)> | The format in which the generated images are returned. This parameter is only supported for the GPT image models. The default value is `png`. SVG output requires a supported image model and project access.  Defaults to `\"png\"`. |  |
**svg** | Option<[**models::SvgParam**](SvgParam.md)> | Selects the author model and reasoning effort for `output_format=svg`. Supply `svg.model` when SVG model selection is enabled. Otherwise, omit this object. SVG output requires `n=1` and cannot use streaming, partial images, or output compression. Use multipart fields such as `svg[model]`. |  |
**output_compression** | Option<**i64**> | The compression level (0-100%) for the generated images. This parameter is only supported for the GPT image models with the `webp` or `jpeg` output formats, and defaults to 100.  Defaults to `100`. |  |
**safety_identifier** | Option<**String**> | A stable identifier for an end user in your organization, used for safety monitoring and abuse detection. Hash a username or email address to avoid sending identifying information. A non-empty value takes precedence over the deprecated `user` parameter. If omitted, null, or empty, the API uses `user` when available. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers). |  |
**user** | Option<**String**> | Deprecated: use `safety_identifier` instead. This parameter remains accepted for backward compatibility and is used when `safety_identifier` is omitted, null, or empty. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers). |  |
**input_fidelity** | Option<[**models::InputFidelity**](InputFidelity.md)> |  |  |
**stream** | Option<**bool**> | Edit the image in streaming mode. Defaults to `false`. See the [Image generation guide](https://developers.openai.com/api/docs/guides/image-generation) for more information.  |  |
**partial_images** | Option<**i64**> | The number of partial images to generate. This parameter is used for streaming responses that return partial images. Value must be between 0 and 3. When set to 0, the response will be a single image sent in one streaming event.  Note that the final image may be sent before the full number of partial images are generated if the full image is generated more quickly.  |  |
**quality** | Option<**String**> | The quality of the image that will be generated for GPT image models. The GPT image models support `low`, `medium`, and `high`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08` snapshots, also support `xhigh` and `max`. Defaults to `auto`.  Defaults to `\\\"auto\\\"`. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_image_edit().configuration(&configuration).image(..).prompt(..).model(..).options(&options).call().await
```

### Return type

[**models::ImagesResponse**](ImagesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data, application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_image_edit_json

> models::ImagesResponse create_image_edit_json(edit_image_body_json_param, options)
Create image edit

Creates an edited or extended image given one or more source images and a prompt. This endpoint supports GPT Image models.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edit_image_body_json_param** | [**EditImageBodyJsonParam**](EditImageBodyJsonParam.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_image_edit_json().configuration(&configuration).edit_image_body_json_param(..).options(&options).call().await
```

### Return type

[**models::ImagesResponse**](ImagesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_image_edit_json_stream

> models::ImageEditStreamEvent create_image_edit_json_stream(edit_image_body_json_param, options)
Create image edit

Creates an edited or extended image given one or more source images and a prompt. This endpoint supports GPT Image models.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edit_image_body_json_param** | [**EditImageBodyJsonParam**](EditImageBodyJsonParam.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_image_edit_json_stream().configuration(&configuration).edit_image_body_json_param(..).options(&options).call().await
```

### Return type

[**models::ImageEditStreamEvent**](ImageEditStreamEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/event-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_image_edit_stream

> models::ImageEditStreamEvent create_image_edit_stream(image, prompt, model, frame_count, frame_duration_ms, complexity, mask, markup, background, n, size, response_format, output_format, svg, output_compression, safety_identifier, user, input_fidelity, stream, partial_images, quality, options)
Create image edit

Creates an edited or extended image given one or more source images and a prompt. This endpoint supports GPT Image models.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image** | **timvw_rust_runtime::Upload** |  | [required] |
**prompt** | **String** | A text description of the desired image(s). The maximum length is 32000 characters for the GPT image models. | [required] |
**model** | **String** |  | [required] |
**frame_count** | Option<**i64**> | Number of frames for GIF output. Accepts integers from 2 through 16. Defaults to 9 when omitted. Only supported with output_format=gif. The requested size must satisfy the selected model's GIF size limits. GIF output does not support streaming, partial images, masks, or custom output compression. Requires access to the GIF preview for the selected model. |  |
**frame_duration_ms** | Option<**i64**> | Requested delay per GIF frame, in milliseconds. Defaults to 200 when omitted. GIF output rounds to the nearest 10 ms and caps the encoded delay at 655,350 ms per frame. Only supported with output_format=gif. |  |
**complexity** | Option<[**models::Complexity**](Complexity.md)> |  |  |
**mask** | Option<**timvw_rust_runtime::Upload**> | An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where `image` should be edited. If there are multiple images provided, the mask will be applied on the first image. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`. |  |
**markup** | Option<**timvw_rust_runtime::Upload**> | A transparent PNG overlaid on the first image to guide the edit. Must match that image's dimensions after orientation and contain transparent pixels. Supports GPT Image 2.5 and 2.6. Cannot be combined with mask. |  |
**background** | Option<**String**> | Set the background of the generated image(s). This parameter is only supported for the GPT image models. Must be one of `transparent`, `opaque`, or `auto` (default value). When `auto` is used, the model will automatically determine the best background for the image.  `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08` snapshots, support `opaque` and `transparent` backgrounds. Transparent backgrounds are available for supported GPT Image models. For `gpt-image-2` and `gpt-image-2-2026-04-21`, this support is in preview. When using `transparent`, set the output format to `png` or `webp`.  Defaults to `\\\"auto\\\"`. |  |
**n** | Option<**i64**> | The number of images to generate. Must be between 1 and 10.  Defaults to `1`. |  |
**size** | Option<[**models::CreateImageEditRequestSize**](CreateImageEditRequestSize.md)> |  |  |
**response_format** | Option<**String**> | Legacy response format parameter for retired image models. Unsupported for GPT image models, which always return base64-encoded images. |  |
**output_format** | Option<[**models::ImagesApiOutputFormat**](ImagesApiOutputFormat.md)> | The format in which the generated images are returned. This parameter is only supported for the GPT image models. The default value is `png`. SVG output requires a supported image model and project access.  Defaults to `\"png\"`. |  |
**svg** | Option<[**models::SvgParam**](SvgParam.md)> | Selects the author model and reasoning effort for `output_format=svg`. Supply `svg.model` when SVG model selection is enabled. Otherwise, omit this object. SVG output requires `n=1` and cannot use streaming, partial images, or output compression. Use multipart fields such as `svg[model]`. |  |
**output_compression** | Option<**i64**> | The compression level (0-100%) for the generated images. This parameter is only supported for the GPT image models with the `webp` or `jpeg` output formats, and defaults to 100.  Defaults to `100`. |  |
**safety_identifier** | Option<**String**> | A stable identifier for an end user in your organization, used for safety monitoring and abuse detection. Hash a username or email address to avoid sending identifying information. A non-empty value takes precedence over the deprecated `user` parameter. If omitted, null, or empty, the API uses `user` when available. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers). |  |
**user** | Option<**String**> | Deprecated: use `safety_identifier` instead. This parameter remains accepted for backward compatibility and is used when `safety_identifier` is omitted, null, or empty. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers). |  |
**input_fidelity** | Option<[**models::InputFidelity**](InputFidelity.md)> |  |  |
**stream** | Option<**bool**> | Edit the image in streaming mode. Defaults to `false`. See the [Image generation guide](https://developers.openai.com/api/docs/guides/image-generation) for more information.  |  |
**partial_images** | Option<**i64**> | The number of partial images to generate. This parameter is used for streaming responses that return partial images. Value must be between 0 and 3. When set to 0, the response will be a single image sent in one streaming event.  Note that the final image may be sent before the full number of partial images are generated if the full image is generated more quickly.  |  |
**quality** | Option<**String**> | The quality of the image that will be generated for GPT image models. The GPT image models support `low`, `medium`, and `high`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08` snapshots, also support `xhigh` and `max`. Defaults to `auto`.  Defaults to `\\\"auto\\\"`. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_image_edit_stream().configuration(&configuration).image(..).prompt(..).model(..).options(&options).call().await
```

### Return type

[**models::ImageEditStreamEvent**](ImageEditStreamEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: text/event-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_image_stream

> models::ImageGenStreamEvent create_image_stream(create_image_request, options)
Create image

Creates an image given a prompt using a GPT Image model. [Learn more](https://developers.openai.com/api/docs/guides/images-vision). 

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_image_request** | [**CreateImageRequest**](CreateImageRequest.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_image_stream().configuration(&configuration).create_image_request(..).options(&options).call().await
```

### Return type

[**models::ImageGenStreamEvent**](ImageGenStreamEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/event-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_image_variation

> models::ImagesResponse create_image_variation(image, model, n, response_format, size, user, options)
Create image variation

This endpoint is retired and no longer available. Use the image edits endpoint with a GPT Image model and a prompt to create a variation of an image. The request and response schemas below describe the legacy contract.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image** | **timvw_rust_runtime::Upload** | The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and square. | [required] |
**model** | Option<**String**> |  |  |
**n** | Option<**i64**> | The number of images to generate. Must be between 1 and 10.  Defaults to `1`. |  |
**response_format** | Option<**String**> | The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated.  Defaults to `\\\"url\\\"`. |  |
**size** | Option<**String**> | The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.  Defaults to `\\\"1024x1024\\\"`. |  |
**user** | Option<**String**> | A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_image_variation().configuration(&configuration).image(..).options(&options).call().await
```

### Return type

[**models::ImagesResponse**](ImagesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

