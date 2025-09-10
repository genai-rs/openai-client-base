# CreateSpeechRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | **String** | One of the available [TTS models](https://platform.openai.com/docs/models#tts): `tts-1`, `tts-1-hd` or `gpt-4o-mini-tts`.  | 
**input** | **String** | The text to generate audio for. The maximum length is 4096 characters. | 
**instructions** | Option<**String**> | Control the voice of your generated audio with additional instructions. Does not work with `tts-1` or `tts-1-hd`. | [optional]
**voice** | **String** | The voice to use when generating the audio. Supported voices are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `onyx`, `nova`, `sage`, `shimmer`, and `verse`. Previews of the voices are available in the [Text to speech guide](https://platform.openai.com/docs/guides/text-to-speech#voice-options). | 
**response_format** | Option<**String**> | The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`. | [optional]
**speed** | Option<**f64**> | The speed of the generated audio. Select a value from `0.25` to `4.0`. `1.0` is the default. | [optional]
**stream_format** | Option<**String**> | The format to stream the audio in. Supported formats are `sse` and `audio`. `sse` is not supported for `tts-1` or `tts-1-hd`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


