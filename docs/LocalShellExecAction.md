# LocalShellExecAction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the local shell action. Always `exec`.  | 
**command** | **Vec<String>** | The command to run.  | 
**timeout_ms** | Option<**i32**> | Optional timeout in milliseconds for the command.  | [optional]
**working_directory** | Option<**String**> | Optional working directory to run the command in.  | [optional]
**env** | **std::collections::HashMap<String, String>** | Environment variables to set for the command.  | 
**user** | Option<**String**> | Optional user to run the command as.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


