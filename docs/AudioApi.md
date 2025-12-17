# \AudioApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_speech**](AudioApi.md#create_speech) | **POST** /audio/speech | Create speech
[**create_transcription**](AudioApi.md#create_transcription) | **POST** /audio/transcriptions | Create transcription
[**create_translation**](AudioApi.md#create_translation) | **POST** /audio/translations | Create translation
[**create_voice**](AudioApi.md#create_voice) | **POST** /audio/voices | Create voice
[**create_voice_consent**](AudioApi.md#create_voice_consent) | **POST** /audio/voice_consents | Create voice consent
[**delete_voice_consent**](AudioApi.md#delete_voice_consent) | **DELETE** /audio/voice_consents/{consent_id} | Delete voice consent
[**get_voice_consent**](AudioApi.md#get_voice_consent) | **GET** /audio/voice_consents/{consent_id} | Retrieve voice consent
[**list_voice_consents**](AudioApi.md#list_voice_consents) | **GET** /audio/voice_consents | List voice consents
[**update_voice_consent**](AudioApi.md#update_voice_consent) | **POST** /audio/voice_consents/{consent_id} | Update voice consent



## create_speech

> std::path::PathBuf create_speech(create_speech_request)
Create speech

Generates audio from the input text.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_speech_request** | [**CreateSpeechRequest**](CreateSpeechRequest.md) |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/octet-stream, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_transcription

> models::CreateTranscription200Response create_transcription(file, model, language, prompt, response_format, temperature, include, timestamp_granularities, stream, chunking_strategy, known_speaker_names, known_speaker_references)
Create transcription

Transcribes audio into the input language.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.  | [required] |
**model** | **String** |  | [required] |
**language** | Option<**String**> | The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve accuracy and latency.  |  |
**prompt** | Option<**String**> | An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://platform.openai.com/docs/guides/speech-to-text#prompting) should match the audio language. This field is not supported when using `gpt-4o-transcribe-diarize`.  |  |
**response_format** | Option<[**models::AudioResponseFormat**](AudioResponseFormat.md)> |  |  |
**temperature** | Option<**f64**> | The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.  |  |
**include** | Option<[**Vec<models::TranscriptionInclude>**](models::TranscriptionInclude.md)> | Additional information to include in the transcription response. `logprobs` will return the log probabilities of the tokens in the response to understand the model's confidence in the transcription. `logprobs` only works with response_format set to `json` and only with the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe`. This field is not supported when using `gpt-4o-transcribe-diarize`.  |  |
**timestamp_granularities** | Option<[**Vec<String>**](String.md)> | The timestamp granularities to populate for this transcription. `response_format` must be set `verbose_json` to use timestamp granularities. Either or both of these options are supported: `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency. This option is not available for `gpt-4o-transcribe-diarize`.  |  |
**stream** | Option<**bool**> | If set to true, the model response data will be streamed to the client as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format). See the [Streaming section of the Speech-to-Text guide](https://platform.openai.com/docs/guides/speech-to-text?lang=curl#streaming-transcriptions) for more information.  Note: Streaming is not supported for the `whisper-1` model and will be ignored.  |  |
**chunking_strategy** | Option<[**models::TranscriptionChunkingStrategy**](TranscriptionChunkingStrategy.md)> |  |  |
**known_speaker_names** | Option<[**Vec<String>**](String.md)> | Optional list of speaker names that correspond to the audio samples provided in `known_speaker_references[]`. Each entry should be a short identifier (for example `customer` or `agent`). Up to 4 speakers are supported.  |  |
**known_speaker_references** | Option<[**Vec<String>**](String.md)> | Optional list of audio samples (as [data URLs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs)) that contain known speaker references matching `known_speaker_names[]`. Each sample must be between 2 and 10 seconds, and can use any of the same input audio formats supported by `file`.  |  |

### Return type

[**models::CreateTranscription200Response**](createTranscription_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_translation

> models::CreateTranslation200Response create_translation(file, model, prompt, response_format, temperature)
Create translation

Translates audio into English.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | The audio file object (not file name) translate, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.  | [required] |
**model** | **String** |  | [required] |
**prompt** | Option<**String**> | An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://platform.openai.com/docs/guides/speech-to-text#prompting) should be in English.  |  |
**response_format** | Option<**String**> | The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.  |  |
**temperature** | Option<**f64**> | The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.  |  |

### Return type

[**models::CreateTranslation200Response**](createTranslation_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_voice

> models::VoiceResource create_voice(name, audio_sample, consent)
Create voice

Creates a custom voice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the new voice. | [required] |
**audio_sample** | **std::path::PathBuf** | The sample audio recording file. Maximum size is 10 MiB.  Supported MIME types: `audio/mpeg`, `audio/wav`, `audio/x-wav`, `audio/ogg`, `audio/aac`, `audio/flac`, `audio/webm`, `audio/mp4`.  | [required] |
**consent** | **String** | The consent recording ID (for example, `cons_1234`). | [required] |

### Return type

[**models::VoiceResource**](VoiceResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
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
**recording** | **std::path::PathBuf** | The consent audio recording file. Maximum size is 10 MiB.  Supported MIME types: `audio/mpeg`, `audio/wav`, `audio/x-wav`, `audio/ogg`, `audio/aac`, `audio/flac`, `audio/webm`, `audio/mp4`.  | [required] |
**language** | **String** | The BCP 47 language tag for the consent phrase (for example, `en-US`). | [required] |

### Return type

[**models::VoiceConsentResource**](VoiceConsentResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
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
**limit** | Option<**i32**> | A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.  |  |[default to 20]

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

