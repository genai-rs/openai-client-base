# ResponseShellCallCommandWsDone

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event, always `response.shell_call_command.done`. | 
**sequence_number** | **i32** | The sequence number of the event that was emitted. | 
**output_index** | **i32** | The index of the output item that was updated. | 
**command_index** | **i32** | The index of the shell command that was completed. | 
**command** | **String** | The final shell command that was emitted. | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the originating `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


