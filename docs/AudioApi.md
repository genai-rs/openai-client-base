# \AudioApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_speech**](AudioApi.md#create_speech) | **POST** /audio/speech | Create speech
[**create_speech_stream**](AudioApi.md#create_speech_stream) | **POST** /audio/speech | Create speech
[**create_transcription**](AudioApi.md#create_transcription) | **POST** /audio/transcriptions | Create transcription
[**create_transcription_stream**](AudioApi.md#create_transcription_stream) | **POST** /audio/transcriptions | Create transcription
[**create_transcription_text**](AudioApi.md#create_transcription_text) | **POST** /audio/transcriptions | Create transcription
[**create_translation**](AudioApi.md#create_translation) | **POST** /audio/translations | Create translation
[**create_translation_text**](AudioApi.md#create_translation_text) | **POST** /audio/translations | Create translation
[**create_voice**](AudioApi.md#create_voice) | **POST** /audio/voices | Create voice
[**create_voice_consent**](AudioApi.md#create_voice_consent) | **POST** /audio/voice_consents | Create voice consent
[**create_voice_multipart**](AudioApi.md#create_voice_multipart) | **POST** /audio/voices | Create voice
[**delete_voice**](AudioApi.md#delete_voice) | **DELETE** /audio/voices/{voice_id} | Delete voice
[**delete_voice_consent**](AudioApi.md#delete_voice_consent) | **DELETE** /audio/voice_consents/{consent_id} | Delete voice consent
[**get_voice_consent**](AudioApi.md#get_voice_consent) | **GET** /audio/voice_consents/{consent_id} | Retrieve voice consent
[**list_voice_consents**](AudioApi.md#list_voice_consents) | **GET** /audio/voice_consents | List voice consents
[**update_voice_consent**](AudioApi.md#update_voice_consent) | **POST** /audio/voice_consents/{consent_id} | Update voice consent



## create_speech

> std::path::PathBuf create_speech(create_speech_request)
Create speech

Generates audio from the input text.  Returns the audio file content, or a stream of audio events.  The `tts-1`, `tts-1-hd`, `gpt-4o-mini-tts`, `gpt-4o-mini-tts-2025-03-20`, and `gpt-4o-mini-tts-2025-12-15` models are deprecated and scheduled to shut down on January 6, 2027. These examples use the existing Speech API; for new integrations, migrate to `gpt-realtime-2.1-mini` through the [Realtime API](https://developers.openai.com/api/docs/guides/realtime). See the [deprecation schedule](https://developers.openai.com/api/docs/deprecations) for migration details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_speech_request** | [**CreateSpeechRequest**](CreateSpeechRequest.md) |  | [required] |

### Return type

**std::path::PathBuf**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/octet-stream, audio/mpeg, audio/aac, audio/opus, audio/flac, audio/pcm, audio/wav, text/event-stream, application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_speech_stream

> models::CreateSpeechResponseStreamEvent create_speech_stream(create_speech_request)
Create speech

Generates audio from the input text.  Returns the audio file content, or a stream of audio events.  The `tts-1`, `tts-1-hd`, `gpt-4o-mini-tts`, `gpt-4o-mini-tts-2025-03-20`, and `gpt-4o-mini-tts-2025-12-15` models are deprecated and scheduled to shut down on January 6, 2027. These examples use the existing Speech API; for new integrations, migrate to `gpt-realtime-2.1-mini` through the [Realtime API](https://developers.openai.com/api/docs/guides/realtime). See the [deprecation schedule](https://developers.openai.com/api/docs/deprecations) for migration details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_speech_request** | [**CreateSpeechRequest**](CreateSpeechRequest.md) |  | [required] |

### Return type

[**models::CreateSpeechResponseStreamEvent**](CreateSpeechResponseStreamEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/event-stream, application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_transcription

> models::CreateTranscription200Response create_transcription(file, model, language, languages, keywords, prompt, response_format, temperature, include, timestamp_granularities, stream, chunking_strategy, known_speaker_names, known_speaker_references)
Create transcription

Transcribes audio into the input language.  Returns a transcription object in `json`, `diarized_json`, or `verbose_json` format, plain text in `text`, `srt`, or `vtt` format, or a stream of transcript events. Supported formats depend on the model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **timvw_rust_runtime::Upload** | The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm. The request must include enough format metadata for the file to be identified. We recommend an extension-bearing filename and an appropriate content type.  | [required] |
**model** | **String** |  | [required] |
**language** | Option<**String**> | The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve accuracy and latency.  |  |
**languages** | Option<**Vec<String>**> | Possible languages of the input audio, in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) format. Supported by `gpt-transcribe`.  |  |
**keywords** | Option<**Vec<String>**> | Words or phrases to guide transcription of the input audio. Supported by `gpt-transcribe`.  |  |
**prompt** | Option<**String**> | An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://developers.openai.com/api/docs/guides/speech-to-text#prompting) should match the audio language. This field is not supported when using `gpt-4o-transcribe-diarize`.  |  |
**response_format** | Option<[**models::AudioResponseFormat**](AudioResponseFormat.md)> |  |  |
**temperature** | Option<**f64**> | The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.  Defaults to `0`. |  |
**include** | Option<[**Vec<models::TranscriptionInclude>**](TranscriptionInclude.md)> | Additional information to include in the transcription response. `logprobs` will return the log probabilities of the tokens in the response to understand the model's confidence in the transcription. `logprobs` only works with response_format set to `json` and only with the models `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `gpt-4o-mini-transcribe-2025-12-15`. This field is not supported when using `gpt-4o-transcribe-diarize`.  |  |
**timestamp_granularities** | Option<**Vec<String>**> | The timestamp granularities to populate for this transcription. `response_format` must be set `verbose_json` to use timestamp granularities. Either or both of these options are supported: `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency. This option is not available for `gpt-4o-transcribe-diarize`.  Defaults to `[\\\"segment\\\"]`. |  |
**stream** | Option<**bool**> | If set to true, the model response data will be streamed to the client as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format). See the [Streaming section of the Speech-to-Text guide](https://developers.openai.com/api/docs/guides/speech-to-text?lang=curl#streaming) for more information.  Note: Streaming is not supported for the `whisper-1` model and will be ignored.  |  |
**chunking_strategy** | Option<[**models::CreateTranscriptionRequestChunkingStrategy**](CreateTranscriptionRequestChunkingStrategy.md)> |  |  |
**known_speaker_names** | Option<**Vec<String>**> | Optional list of speaker names that correspond to the audio samples provided in `known_speaker_references[]`. Each entry should be a short identifier (for example `customer` or `agent`). Up to 4 speakers are supported.  |  |
**known_speaker_references** | Option<**Vec<String>**> | Optional list of audio samples (as [data URLs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs)) that contain known speaker references matching `known_speaker_names[]`. Each sample must be between 2 and 10 seconds, and can use any of the same input audio formats supported by `file`.  |  |

### Return type

[**models::CreateTranscription200Response**](CreateTranscription200Response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json, text/plain, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_transcription_stream

> models::CreateTranscriptionResponseStreamEvent create_transcription_stream(file, model, language, languages, keywords, prompt, response_format, temperature, include, timestamp_granularities, stream, chunking_strategy, known_speaker_names, known_speaker_references)
Create transcription

Transcribes audio into the input language.  Returns a transcription object in `json`, `diarized_json`, or `verbose_json` format, plain text in `text`, `srt`, or `vtt` format, or a stream of transcript events. Supported formats depend on the model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **timvw_rust_runtime::Upload** | The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm. The request must include enough format metadata for the file to be identified. We recommend an extension-bearing filename and an appropriate content type.  | [required] |
**model** | **String** |  | [required] |
**language** | Option<**String**> | The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve accuracy and latency.  |  |
**languages** | Option<**Vec<String>**> | Possible languages of the input audio, in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) format. Supported by `gpt-transcribe`.  |  |
**keywords** | Option<**Vec<String>**> | Words or phrases to guide transcription of the input audio. Supported by `gpt-transcribe`.  |  |
**prompt** | Option<**String**> | An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://developers.openai.com/api/docs/guides/speech-to-text#prompting) should match the audio language. This field is not supported when using `gpt-4o-transcribe-diarize`.  |  |
**response_format** | Option<[**models::AudioResponseFormat**](AudioResponseFormat.md)> |  |  |
**temperature** | Option<**f64**> | The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.  Defaults to `0`. |  |
**include** | Option<[**Vec<models::TranscriptionInclude>**](TranscriptionInclude.md)> | Additional information to include in the transcription response. `logprobs` will return the log probabilities of the tokens in the response to understand the model's confidence in the transcription. `logprobs` only works with response_format set to `json` and only with the models `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `gpt-4o-mini-transcribe-2025-12-15`. This field is not supported when using `gpt-4o-transcribe-diarize`.  |  |
**timestamp_granularities** | Option<**Vec<String>**> | The timestamp granularities to populate for this transcription. `response_format` must be set `verbose_json` to use timestamp granularities. Either or both of these options are supported: `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency. This option is not available for `gpt-4o-transcribe-diarize`.  Defaults to `[\\\"segment\\\"]`. |  |
**stream** | Option<**bool**> | If set to true, the model response data will be streamed to the client as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format). See the [Streaming section of the Speech-to-Text guide](https://developers.openai.com/api/docs/guides/speech-to-text?lang=curl#streaming) for more information.  Note: Streaming is not supported for the `whisper-1` model and will be ignored.  |  |
**chunking_strategy** | Option<[**models::CreateTranscriptionRequestChunkingStrategy**](CreateTranscriptionRequestChunkingStrategy.md)> |  |  |
**known_speaker_names** | Option<**Vec<String>**> | Optional list of speaker names that correspond to the audio samples provided in `known_speaker_references[]`. Each entry should be a short identifier (for example `customer` or `agent`). Up to 4 speakers are supported.  |  |
**known_speaker_references** | Option<**Vec<String>**> | Optional list of audio samples (as [data URLs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs)) that contain known speaker references matching `known_speaker_names[]`. Each sample must be between 2 and 10 seconds, and can use any of the same input audio formats supported by `file`.  |  |

### Return type

[**models::CreateTranscriptionResponseStreamEvent**](CreateTranscriptionResponseStreamEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: text/event-stream, application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_transcription_text

> String create_transcription_text(file, model, language, languages, keywords, prompt, response_format, temperature, include, timestamp_granularities, stream, chunking_strategy, known_speaker_names, known_speaker_references)
Create transcription

Transcribes audio into the input language.  Returns a transcription object in `json`, `diarized_json`, or `verbose_json` format, plain text in `text`, `srt`, or `vtt` format, or a stream of transcript events. Supported formats depend on the model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **timvw_rust_runtime::Upload** | The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm. The request must include enough format metadata for the file to be identified. We recommend an extension-bearing filename and an appropriate content type.  | [required] |
**model** | **String** |  | [required] |
**language** | Option<**String**> | The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve accuracy and latency.  |  |
**languages** | Option<**Vec<String>**> | Possible languages of the input audio, in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) format. Supported by `gpt-transcribe`.  |  |
**keywords** | Option<**Vec<String>**> | Words or phrases to guide transcription of the input audio. Supported by `gpt-transcribe`.  |  |
**prompt** | Option<**String**> | An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://developers.openai.com/api/docs/guides/speech-to-text#prompting) should match the audio language. This field is not supported when using `gpt-4o-transcribe-diarize`.  |  |
**response_format** | Option<[**models::AudioResponseFormat**](AudioResponseFormat.md)> |  |  |
**temperature** | Option<**f64**> | The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.  Defaults to `0`. |  |
**include** | Option<[**Vec<models::TranscriptionInclude>**](TranscriptionInclude.md)> | Additional information to include in the transcription response. `logprobs` will return the log probabilities of the tokens in the response to understand the model's confidence in the transcription. `logprobs` only works with response_format set to `json` and only with the models `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `gpt-4o-mini-transcribe-2025-12-15`. This field is not supported when using `gpt-4o-transcribe-diarize`.  |  |
**timestamp_granularities** | Option<**Vec<String>**> | The timestamp granularities to populate for this transcription. `response_format` must be set `verbose_json` to use timestamp granularities. Either or both of these options are supported: `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency. This option is not available for `gpt-4o-transcribe-diarize`.  Defaults to `[\\\"segment\\\"]`. |  |
**stream** | Option<**bool**> | If set to true, the model response data will be streamed to the client as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format). See the [Streaming section of the Speech-to-Text guide](https://developers.openai.com/api/docs/guides/speech-to-text?lang=curl#streaming) for more information.  Note: Streaming is not supported for the `whisper-1` model and will be ignored.  |  |
**chunking_strategy** | Option<[**models::CreateTranscriptionRequestChunkingStrategy**](CreateTranscriptionRequestChunkingStrategy.md)> |  |  |
**known_speaker_names** | Option<**Vec<String>**> | Optional list of speaker names that correspond to the audio samples provided in `known_speaker_references[]`. Each entry should be a short identifier (for example `customer` or `agent`). Up to 4 speakers are supported.  |  |
**known_speaker_references** | Option<**Vec<String>**> | Optional list of audio samples (as [data URLs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs)) that contain known speaker references matching `known_speaker_names[]`. Each sample must be between 2 and 10 seconds, and can use any of the same input audio formats supported by `file`.  |  |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_translation

> models::CreateTranslation200Response create_translation(file, model, prompt, response_format, temperature)
Create translation

Translates audio into English.  This endpoint supports only `whisper-1`. The endpoint and model are deprecated and scheduled to shut down on February 26, 2027. Migrate to `/v1/realtime/translations`, which requires a separate integration. See the [deprecation schedule](https://developers.openai.com/api/docs/deprecations) for details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **timvw_rust_runtime::Upload** | The audio file object (not file name) translate, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm. The request must include enough format metadata for the file to be identified. We recommend an extension-bearing filename and an appropriate content type.  | [required] |
**model** | **String** |  | [required] |
**prompt** | Option<**String**> | An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://developers.openai.com/api/docs/guides/speech-to-text#prompting) should be in English.  |  |
**response_format** | Option<**String**> | The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.  Defaults to `\\\"json\\\"`. |  |
**temperature** | Option<**f64**> | The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.  Defaults to `0`. |  |

### Return type

[**models::CreateTranslation200Response**](CreateTranslation200Response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_translation_text

> String create_translation_text(file, model, prompt, response_format, temperature)
Create translation

Translates audio into English.  This endpoint supports only `whisper-1`. The endpoint and model are deprecated and scheduled to shut down on February 26, 2027. Migrate to `/v1/realtime/translations`, which requires a separate integration. See the [deprecation schedule](https://developers.openai.com/api/docs/deprecations) for details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **timvw_rust_runtime::Upload** | The audio file object (not file name) translate, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm. The request must include enough format metadata for the file to be identified. We recommend an extension-bearing filename and an appropriate content type.  | [required] |
**model** | **String** |  | [required] |
**prompt** | Option<**String**> | An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://developers.openai.com/api/docs/guides/speech-to-text#prompting) should be in English.  |  |
**response_format** | Option<**String**> | The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.  Defaults to `\\\"json\\\"`. |  |
**temperature** | Option<**f64**> | The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.  Defaults to `0`. |  |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_voice

> models::VoiceResource create_voice(create_voice_prompt_request)
Create voice

Creates a voice from a text prompt or from a consent recording and an audio sample.  For prompt-based creation, send `type: \"prompt\"` with a `name` and `prompt` as JSON or multipart form data. For creation from an audio sample, send `type: \"audio_sample\"` with a `name`, `audio_sample`, and `consent` recording ID as multipart form data. The type defaults to `audio_sample` when omitted.  Returns the saved voice's metadata. Voices created from text prompts are supported only in Live, not in Realtime or the speech endpoint. The response does not include preview audio. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_voice_prompt_request** | [**CreateVoicePromptRequest**](CreateVoicePromptRequest.md) |  | [required] |

### Return type

[**models::VoiceResource**](VoiceResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_voice_consent

> models::VoiceConsentResource create_voice_consent(name, recording, language)
Create voice consent

Upload a voice consent recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The label to use for this consent recording. | [required] |
**recording** | **timvw_rust_runtime::Upload** | The consent audio recording file. Maximum size is 10 MiB.  Supported MIME types: `audio/mpeg`, `audio/wav`, `audio/x-wav`, `audio/ogg`, `audio/aac`, `audio/flac`, `audio/webm`, `audio/mp4`.  | [required] |
**language** | **String** | The BCP 47 language tag for the consent phrase (for example, `en-US`). | [required] |

### Return type

[**models::VoiceConsentResource**](VoiceConsentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_voice_multipart

> models::VoiceResource create_voice_multipart(r#type, name, audio_sample, consent, prompt, script_hint, model)
Create voice

Creates a voice from a text prompt or from a consent recording and an audio sample.  For prompt-based creation, send `type: \"prompt\"` with a `name` and `prompt` as JSON or multipart form data. For creation from an audio sample, send `type: \"audio_sample\"` with a `name`, `audio_sample`, and `consent` recording ID as multipart form data. The type defaults to `audio_sample` when omitted.  Returns the saved voice's metadata. Voices created from text prompts are supported only in Live, not in Realtime or the speech endpoint. The response does not include preview audio. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Set to `prompt` to create a voice from a text description. |  |
**name** | Option<**String**> | The name of the new voice. |  |
**audio_sample** | Option<**timvw_rust_runtime::Upload**> | The sample audio recording file. Maximum size is 10 MiB.  Supported MIME types: `audio/mpeg`, `audio/wav`, `audio/x-wav`, `audio/ogg`, `audio/aac`, `audio/flac`, `audio/webm`, `audio/mp4`.  |  |
**consent** | Option<**String**> | The consent recording ID (for example, `cons_1234`). |  |
**prompt** | Option<**String**> | A description of the desired voice. Must not contain only whitespace. |  |
**script_hint** | Option<**String**> | Optional text for the voice to speak during creation. If omitted, a script is generated from the prompt. Must not be blank after trimming whitespace; scripts that are too short are rejected. |  |
**model** | Option<**String**> | The voice creation model to use. Defaults to `auto`.  Defaults to `\\\"auto\\\"`. |  |

### Return type

[**models::VoiceResource**](VoiceResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_voice

> models::VoiceDeletedResource delete_voice(voice_id)
Delete voice

Deletes a voice created from a text prompt or an audio sample.  Requires voice write permission and access to the voice's project. Access to voices created from recordings is limited to eligible customers. See the [custom voices guide](https://developers.openai.com/api/docs/guides/custom-voices).  Deleting a voice does not delete its consent recording. Deleting an already deleted voice returns a 404 error. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**voice_id** | **String** | The ID of the voice to delete. | [required] |

### Return type

[**models::VoiceDeletedResource**](VoiceDeletedResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_voice_consent

> models::VoiceConsentDeletedResource delete_voice_consent(consent_id)
Delete voice consent

Deletes a voice consent recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | The ID of the consent recording to delete. | [required] |

### Return type

[**models::VoiceConsentDeletedResource**](VoiceConsentDeletedResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_consent

> models::VoiceConsentResource get_voice_consent(consent_id)
Retrieve voice consent

Retrieves a voice consent recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | The ID of the consent recording to retrieve. | [required] |

### Return type

[**models::VoiceConsentResource**](VoiceConsentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_voice_consents

> models::VoiceConsentListResource list_voice_consents(after, limit)
List voice consents

Returns a list of voice consent recordings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**String**> | A cursor for use in pagination. `after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with obj_foo, your subsequent call can include after=obj_foo in order to fetch the next page of the list.  |  |
**limit** | Option<**i64**> | A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.  Defaults to `20`. |  |[default to 20]

### Return type

[**models::VoiceConsentListResource**](VoiceConsentListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_voice_consent

> models::VoiceConsentResource update_voice_consent(consent_id, update_voice_consent_request)
Update voice consent

Updates a voice consent recording (metadata only).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | The ID of the consent recording to update. | [required] |
**update_voice_consent_request** | [**UpdateVoiceConsentRequest**](UpdateVoiceConsentRequest.md) |  | [required] |

### Return type

[**models::VoiceConsentResource**](VoiceConsentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

