# BetaResponseShellCallCommandDeltaStreamingEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event, always `response.shell_call_command.delta`. | 
**sequence_number** | **i32** | The sequence number of the event that was emitted. | 
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**output_index** | **i32** | The index of the output item that was updated. | 
**command_index** | **i32** | The index of the shell command that was updated. | 
**delta** | **String** | The shell command delta that was appended. | 
**obfuscation** | Option<**String**> | An obfuscation string that was added to pad the event payload. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


