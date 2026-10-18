# CreateVoicePromptRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Set to `prompt` to create a voice from a text description. | 
**name** | **String** | The name of the new voice. | 
**prompt** | **String** | A description of the desired voice. Must not contain only whitespace. | 
**script_hint** | Option<**String**> | Optional text for the voice to speak during creation. If omitted, a script is generated from the prompt. Must not be blank after trimming whitespace; scripts that are too short are rejected. | [optional]
**model** | Option<**String**> | The voice creation model to use. Defaults to `auto`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


