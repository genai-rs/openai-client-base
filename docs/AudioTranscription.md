# AudioTranscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> | The model to use for transcription. Current options are `whisper-1`, `gpt-4o-transcribe-latest`, `gpt-4o-mini-transcribe`, and `gpt-4o-transcribe`.  | [optional]
**language** | Option<**String**> | The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve accuracy and latency.  | [optional]
**prompt** | Option<**String**> | An optional text to guide the model's style or continue a previous audio segment. For `whisper-1`, the [prompt is a list of keywords](https://platform.openai.com/docs/guides/speech-to-text#prompting). For `gpt-4o-transcribe` models, the prompt is a free text string, for example \"expect words related to technology\".  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


