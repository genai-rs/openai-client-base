# RealtimeConversationItemMessageUserContentInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The content type (`input_text`, `input_audio`, or `input_image`). | [optional]
**text** | Option<**String**> | The text content (for `input_text`). | [optional]
**audio** | Option<**String**> | Base64-encoded audio bytes (for `input_audio`), these will be parsed as the format specified in the session input audio type configuration. This defaults to PCM 16-bit 24kHz mono if not specified. | [optional]
**image_url** | Option<**String**> | Base64-encoded image bytes (for `input_image`) as a data URI. For example `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAA...`. Supported formats are PNG and JPEG. | [optional]
**detail** | Option<**String**> | The detail level of the image (for `input_image`). `auto` will default to `high`. | [optional]
**transcript** | Option<**String**> | Transcript of the audio (for `input_audio`). This is not sent to the model, but will be attached to the message item for reference. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


