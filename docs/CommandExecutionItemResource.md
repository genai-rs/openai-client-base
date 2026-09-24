# CommandExecutionItemResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The item type. Always `command_execution`. | 
**id** | **String** | The ID of the command execution item. | 
**turn_id** | **String** | The ID of the turn that contains this item. | 
**command** | **String** | The command that was executed. | 
**cwd** | Option<**String**> | The working directory used to execute the command. | 
**status** | [**models::FunctionCallStatusResource**](FunctionCallStatusResource.md) |  | 
**output** | Option<**String**> | The command output, if available. | 
**exit_code** | Option<**i64**> | The process exit code, if the command completed. | 
**duration_ms** | Option<**i64**> | The command duration in milliseconds. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


