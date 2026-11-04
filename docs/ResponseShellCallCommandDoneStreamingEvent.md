# ResponseShellCallCommandDoneStreamingEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the event, always `response.shell_call_command.done`. (enum: response.shell_call_command.done) | 
**sequence_number** | **i64** | The sequence number of the event that was emitted. | 
**output_index** | **i64** | The index of the output item that was updated. | 
**command_index** | **i64** | The index of the shell command that was completed. | 
**command** | **String** | The final shell command that was emitted. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


