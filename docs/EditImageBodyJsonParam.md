# EditImageBodyJsonParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> | The GPT image model to use for image editing, including `gpt-image-2`, its dated snapshot `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and `gpt-image-2.5-flare-2026-09-08`. Defaults to `gpt-image-2.5-sunburst`. | [optional]
**images** | [**Vec<models::ImageRefParam>**](ImageRefParam.md) | Input image references to edit. For GPT image models, you can provide up to 16 images.  | 
**mask** | Option<[**models::ImageRefParam**](ImageRefParam.md)> |  | [optional]
**prompt** | **String** | A text description of the desired image edit. | 
**n** | Option<**i32**> | The number of edited images to generate. | [optional]
**quality** | Option<**Quality**> | Output quality for GPT image models. The GPT image models support `low`, `medium`, and `high`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08` snapshots, also support `xhigh` and `max`. Defaults to `auto`.  (enum: low, medium, high, xhigh, max, auto) | [optional]
**input_fidelity** | Option<**InputFidelity**> | Control how much effort the model will exert to match the style and features, especially facial features, of input images. Supports `high` and `low` on `gpt-image-1` and `gpt-image-1.5`; `gpt-image-1-mini` supports only `low`. For `gpt-image-2`, omit this parameter. Defaults to `low` on supported models. (enum: high, low) | [optional]
**size** | Option<[**models::EditImageBodyJsonParamSize**](EditImageBodyJsonParamSize.md)> |  | [optional]
**safety_identifier** | Option<**String**> | A stable identifier for an end user in your organization, used for safety monitoring and abuse detection. Hash a username or email address to avoid sending identifying information. A non-empty value takes precedence over the deprecated `user` parameter. If omitted, null, or empty, the API uses `user` when available. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers). | [optional]
**user** | Option<**String**> | Deprecated: use `safety_identifier` instead. This parameter remains accepted for backward compatibility and is used when `safety_identifier` is omitted, null, or empty. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers). | [optional]
**output_format** | Option<**OutputFormat**> | Output image format. Supported for GPT image models. (enum: png, jpeg, webp) | [optional]
**output_compression** | Option<**i32**> | Compression level for `jpeg` or `webp` output. | [optional]
**moderation** | Option<**Moderation**> | Moderation level for GPT image models. (enum: low, auto) | [optional]
**background** | Option<**Background**> | Set the background of the generated image output. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08` snapshots, support `opaque` and `transparent` backgrounds. Transparent backgrounds are available for supported GPT Image models. For `gpt-image-2` and `gpt-image-2-2026-04-21`, this support is in preview. When using `transparent`, set the output format to `png` or `webp`. (enum: transparent, opaque, auto) | [optional]
**stream** | Option<**bool**> | Stream partial image results as events. | [optional]
**partial_images** | Option<**i32**> | The number of partial images to generate. This parameter is used for streaming responses that return partial images. Value must be between 0 and 3. When set to 0, the response will be a single image sent in one streaming event.  Note that the final image may be sent before the full number of partial images are generated if the full image is generated more quickly.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


