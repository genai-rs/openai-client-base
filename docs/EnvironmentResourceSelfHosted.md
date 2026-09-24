# EnvironmentResourceSelfHosted

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `self_hosted`. | 
**remote_url** | **String** | Pass this URL unchanged to `codex exec-server --remote` when connecting this environment. | 
**id** | **String** | The public ID of the environment. | 
**workspace_directory** | **String** | The absolute project directory inside the environment. Defaults to `/workspace`. | 
**capability_directories** | **Vec<String>** | Directories that contain capabilities exposed to the agent. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


