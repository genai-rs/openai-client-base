# BetaImageGenTool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the image generation tool. Always `image_generation`.  (enum: image_generation) | 
**model** | Option<**String**> | ID of the model to use | [optional]
**quality** | Option<**Quality**> | The quality of the generated image. The GPT image models support `low`, `medium`, and `high`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08` snapshots, also support `xhigh` and `max`. Default: `auto`.  (enum: low, medium, high, xhigh, max, auto) | [optional]
**size** | Option<[**models::ImageGenToolSize**](ImageGenToolSize.md)> |  | [optional]
**output_format** | Option<**OutputFormat**> | The output format of the generated image. One of `png`, `webp`, or `jpeg`. Default: `png`.  (enum: png, webp, jpeg) | [optional]
**output_compression** | Option<**i32**> | Compression level for the output image. Default: 100.  | [optional]
**moderation** | Option<**Moderation**> | Moderation level for the generated image. Default: `auto`.  (enum: auto, low) | [optional]
**background** | Option<**Background**> | Set the background of the generated image. One of `transparent`, `opaque`, or `auto`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08` snapshots, support `opaque` and `transparent` backgrounds. Transparent backgrounds are available for supported GPT Image models. For `gpt-image-2` and `gpt-image-2-2026-04-21`, this support is in preview. When using `transparent`, set the output format to `png` or `webp`. Default: `auto`.  (enum: transparent, opaque, auto) | [optional]
**input_fidelity** | Option<[**models::BetaInputFidelity**](BetaInputFidelity.md)> |  | [optional]
**input_image_mask** | Option<[**models::ImageGenToolInputImageMask**](ImageGenToolInputImageMask.md)> |  | [optional]
**partial_images** | Option<**i32**> | Number of partial images to generate in streaming mode, from 0 (default value) to 3.  | [optional]
**action** | Option<[**models::BetaImageGenActionEnum**](BetaImageGenActionEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


