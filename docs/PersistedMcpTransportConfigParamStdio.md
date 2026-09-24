# PersistedMcpTransportConfigParamStdio

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `stdio`. | 
**command** | **String** | The command used to start the MCP server. | 
**args** | Option<**Vec<String>**> | Arguments passed to the MCP server command. | [optional]
**cwd** | **String** | The working directory used to start the MCP server. | 
**env_vars** | Option<**Vec<String>**> | Environment variable names to inherit from the selected execution environment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


