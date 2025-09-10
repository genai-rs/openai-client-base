# RealtimeConversationItemMessageAssistantContentInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The content type, `output_text` or `output_audio` depending on the session `output_modalities` configuration. | [optional]
**text** | Option<**String**> | The text content. | [optional]
**audio** | Option<**String**> | Base64-encoded audio bytes, these will be parsed as the format specified in the session output audio type configuration. This defaults to PCM 16-bit 24kHz mono if not specified. | [optional]
**transcript** | Option<**String**> | The transcript of the audio content, this will always be present if the output type is `audio`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


