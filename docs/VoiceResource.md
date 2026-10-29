# VoiceResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | The object type, which is always `audio.voice`. (enum: audio.voice) | 
**id** | **String** | The voice identifier, which can be referenced in API endpoints. | 
**r#type** | **Type** | How the voice was created. (enum: audio_sample, prompt) | 
**prompt** | Option<**String**> | The original text prompt used to create the voice. Present for prompt-generated voices with a stored prompt; omitted for audio-sample voices and older voices without a stored prompt. | [optional]
**name** | **String** | The name of the voice. | 
**created_at** | **i32** | The Unix timestamp (in seconds) for when the voice was created. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


