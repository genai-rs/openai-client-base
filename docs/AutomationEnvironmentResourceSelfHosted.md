# AutomationEnvironmentResourceSelfHosted

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `self_hosted`.  Defaults to `\"self_hosted\"`. (enum: self_hosted) | 
**environment_id** | Option<**String**> | An existing registration, or null to create a session-owned environment. | 
**workspace_directory** | **String** | The absolute project directory in the environment. | 
**capability_directories** | **Vec<String>** | Directories that expose capabilities to the agent. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


