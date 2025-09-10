# Image

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**b64_json** | Option<**String**> | The base64-encoded JSON of the generated image. Default value for `gpt-image-1`, and only present if `response_format` is set to `b64_json` for `dall-e-2` and `dall-e-3`. | [optional]
**url** | Option<**String**> | When using `dall-e-2` or `dall-e-3`, the URL of the generated image if `response_format` is set to `url` (default value). Unsupported for `gpt-image-1`. | [optional]
**revised_prompt** | Option<**String**> | For `dall-e-3` only, the revised prompt that was used to generate the image. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


