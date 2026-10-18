# EditImageBodyJsonParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> | The GPT image model to use for image editing, including `gpt-image-2`, its dated snapshot `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and `gpt-image-2.5-flare-2026-09-08`. Defaults to `gpt-image-2.5-sunburst`. | [optional]
**images** | [**Vec<models::ImageRefParam>**](ImageRefParam.md) | Input image references to edit. For GPT image models, you can provide up to 16 images.  | 
**mask** | Option<[**models::ImageRefParam**](ImageRefParam.md)> |  | [optional]
**prompt** | **String** | A text description of the desired image edit. | 
**n** | Option<**i32**> |  | [optional]
**quality** | Option<**String**> |  | [optional]
**input_fidelity** | Option<**String**> |  | [optional]
**size** | Option<[**models::EditImageBodyJsonParamSize**](EditImageBodyJsonParam_size.md)> |  | [optional]
**safety_identifier** | Option<**String**> | A stable identifier for an end user in your organization, used for safety monitoring and abuse detection. Hash a username or email address to avoid sending identifying information. A non-empty value takes precedence over the deprecated `user` parameter. If omitted, null, or empty, the API uses `user` when available. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers). | [optional]
**user** | Option<**String**> | Deprecated: use `safety_identifier` instead. This parameter remains accepted for backward compatibility and is used when `safety_identifier` is omitted, null, or empty. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers). | [optional]
**output_format** | Option<**String**> |  | [optional]
**output_compression** | Option<**i32**> |  | [optional]
**moderation** | Option<**String**> |  | [optional]
**background** | Option<**String**> |  | [optional]
**stream** | Option<**bool**> |  | [optional]
**partial_images** | Option<**i32**> | The number of partial images to generate. This parameter is used for streaming responses that return partial images. Value must be between 0 and 3. When set to 0, the response will be a single image sent in one streaming event.  Note that the final image may be sent before the full number of partial images are generated if the full image is generated more quickly.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


