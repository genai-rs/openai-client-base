# ImageGenTool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the image generation tool. Always `image_generation`.  (enum: image_generation) | 
**model** | Option<**String**> | ID of the model to use | [optional]
**complexity** | Option<**Complexity**> | Computation spent planning the image, independently of rendering quality. Overrides the outer response's `reasoning.effort` for image generation. When omitted, inherits a supported outer reasoning effort or uses the image model's configured default. For SVG output, controls the reference image, not the SVG author's reasoning.  (enum: low, medium, high) | [optional]
**quality** | Option<**Quality**> | The quality of the generated image. The GPT image models support `low`, `medium`, and `high`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08` snapshots, also support `xhigh` and `max`. Default: `auto`.  Defaults to `\"auto\"`. (enum: low, medium, high, xhigh, max, auto) | [optional]
**size** | Option<[**models::ImageGenToolSize**](ImageGenToolSize.md)> |  | [optional]
**output_format** | Option<**OutputFormat**> | The output format of the generated image. One of `png`, `webp`, `jpeg`, `gif`, or `svg`. Default: `png`. GIF and SVG output require a model that supports the selected format. GIF output does not support partial image streaming (`partial_images` greater than `0`), `input_image_mask`, or custom `output_compression`. Responses lifecycle events can still be streamed for GIF output.  Defaults to `\"png\"`. (enum: png, webp, jpeg, gif, svg) | [optional]
**frame_count** | Option<**i64**> | Number of GIF frames. Requires `output_format` to be `gif`. Default: 9.  Defaults to `9`. | [optional]
**frame_duration_ms** | Option<**i64**> | Duration of each GIF frame in milliseconds, rounded to 10 ms. Requires `output_format` to be `gif`. Default: 200.  Defaults to `200`. | [optional]
**svg** | Option<[**models::SvgParam**](SvgParam.md)> |  | [optional]
**output_compression** | Option<**i64**> | Compression level for the output image. Default: 100.  Defaults to `100`. | [optional]
**moderation** | Option<**Moderation**> | Moderation level for the generated image. Default: `auto`.  Defaults to `\"auto\"`. (enum: auto, low) | [optional]
**background** | Option<**Background**> | Set the background of the generated image. One of `transparent`, `opaque`, or `auto`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08` snapshots, support `opaque` and `transparent` backgrounds. Transparent backgrounds are available for supported GPT Image models. For `gpt-image-2` and `gpt-image-2-2026-04-21`, this support is in preview. When using `transparent`, set the output format to `png` or `webp`. Default: `auto`.  Defaults to `\"auto\"`. (enum: transparent, opaque, auto) | [optional]
**input_fidelity** | Option<[**models::InputFidelity**](InputFidelity.md)> |  | [optional]
**input_image_mask** | Option<[**models::ImageGenToolInputImageMask**](ImageGenToolInputImageMask.md)> |  | [optional]
**partial_images** | Option<**i64**> | Number of partial images to generate in streaming mode, from 0 (default value) to 3.  Defaults to `0`. | [optional]
**action** | Option<[**models::ImageGenActionEnum**](ImageGenActionEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


