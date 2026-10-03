# Image

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**b64_json** | Option<**String**> | The base64-encoded JSON of the generated image. Returned by default for GPT image models, or when `response_format` is set to `b64_json` for models that support that parameter. | [optional]
**url** | Option<**String**> | The URL of the generated image when `response_format` is set to `url` for models that support that parameter. Unsupported for GPT image models. | [optional]
**revised_prompt** | Option<**String**> | The revised prompt used to generate the image, for models that support prompt revision. Not returned by GPT image models. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


