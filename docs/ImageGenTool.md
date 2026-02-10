# ImageGenTool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the image generation tool. Always `image_generation`.  | 
**model** | Option<**String**> | ID of the model to use | [optional]
**quality** | Option<**String**> | The quality of the generated image. One of `low`, `medium`, `high`, or `auto`. Default: `auto`.  | [optional]
**size** | Option<**String**> | The size of the generated image. One of `1024x1024`, `1024x1536`, `1536x1024`, or `auto`. Default: `auto`.  | [optional]
**output_format** | Option<**String**> | The output format of the generated image. One of `png`, `webp`, or `jpeg`. Default: `png`.  | [optional]
**output_compression** | Option<**i32**> | Compression level for the output image. Default: 100.  | [optional]
**moderation** | Option<**String**> | Moderation level for the generated image. Default: `auto`.  | [optional]
**background** | Option<**String**> | Background type for the generated image. One of `transparent`, `opaque`, or `auto`. Default: `auto`.  | [optional]
**input_fidelity** | Option<[**models::InputFidelity**](InputFidelity.md)> |  | [optional]
**input_image_mask** | Option<[**models::ImageGenToolInputImageMask**](ImageGenTool_input_image_mask.md)> |  | [optional]
**partial_images** | Option<**i32**> | Number of partial images to generate in streaming mode, from 0 (default value) to 3.  | [optional]
**action** | Option<[**models::ImageGenActionEnum**](ImageGenActionEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


