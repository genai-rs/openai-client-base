# BetaResponseShellCallOutputContentWsDone

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event, always `response.shell_call_output_content.done`. | 
**sequence_number** | **i32** | The sequence number of the event that was emitted. | 
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**item_id** | **String** | The ID of the output item that was updated. | 
**output_index** | **i32** | The index of the output item that was updated. | 
**command_index** | **i32** | The index of the shell command that produced output. | 
**output** | [**Vec<models::BetaFunctionShellCallOutputContent>**](BetaFunctionShellCallOutputContent.md) | The output contents emitted for the shell command. | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the originating `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


