# BetaResponseShellCallCommandAddedStreamingEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event, always `response.shell_call_command.added`. | 
**sequence_number** | **i32** | The sequence number of the event that was emitted. | 
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**output_index** | **i32** | The index of the output item that was updated. | 
**command_index** | **i32** | The index of the shell command that was added. | 
**command** | **String** | The shell command that was added. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


