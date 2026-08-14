# AudioTranscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> | The model to use for transcription. Current options are `whisper-1`, `gpt-transcribe`, `gpt-live-transcribe`, `gpt-4o-mini-transcribe`, `gpt-4o-mini-transcribe-2025-12-15`, `gpt-4o-transcribe`, `gpt-4o-transcribe-diarize`, and `gpt-realtime-whisper`. Use `gpt-4o-transcribe-diarize` when you need diarization with speaker labels.  | [optional]
**language** | Option<**String**> | The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve accuracy and latency.  | [optional]
**languages** | Option<**Vec<String>**> | Possible languages of the input audio, in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) format. Supported by `gpt-transcribe` and `gpt-live-transcribe`.  | [optional]
**keywords** | Option<**Vec<String>**> | Words or phrases to guide transcription of the input audio. Supported by `gpt-transcribe` and `gpt-live-transcribe`.  | [optional]
**prompt** | Option<**String**> | An optional text to guide the model's style or continue a previous audio segment. For `whisper-1`, the [prompt is a list of keywords](/docs/guides/speech-to-text#prompting). For `gpt-4o-transcribe` models (excluding `gpt-4o-transcribe-diarize`), the prompt is a free text string, for example \"expect words related to technology\". Prompt is not supported with `gpt-realtime-whisper` in GA Realtime sessions.  | [optional]
**delay** | Option<**String**> | Controls how long the model waits before emitting transcription text. Higher values can improve transcription accuracy at the cost of latency. Only supported with `gpt-realtime-whisper` in GA Realtime sessions.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


