# CreateEnvironmentParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `openai_hosted`. (enum: openai_hosted) | 
**packages** | Option<[**models::EnvironmentPackagesParam**](EnvironmentPackagesParam.md)> | Packages to install in the environment. Defaults to empty package lists. | [optional]
**setup_commands** | Option<[**Vec<models::SetupCommandParam>**](SetupCommandParam.md)> | Ordered, confidential setup commands. Command bodies are never returned. | [optional]
**network** | Option<[**models::NetworkPolicyParam**](NetworkPolicyParam.md)> | Network access policy for the environment. If omitted, the API version determines whether network access is enabled or disabled. | [optional]
**desktop** | Option<[**models::DesktopParam**](DesktopParam.md)> | Desktop provisioning. Omission or null inherits the template setting, or defaults to disabled. | [optional]
**env** | Option<**std::collections::HashMap<String, String>**> | Environment variables made available to the agent. | [optional]
**capability_directories** | Option<**Vec<String>**> | Directories that contain capabilities exposed to the agent. Defaults to an empty list. | [optional]
**skills** | Option<[**Vec<models::HostedSkillParam>**](HostedSkillParam.md)> | Skills referenced by ID or provided as inline ZIP archives. Defaults to an empty list. | [optional]
**plugins** | Option<[**Vec<models::HostedPluginParam>**](HostedPluginParam.md)> | Plugins provided as inline ZIP archives. Defaults to an empty list. | [optional]
**files** | Option<[**Vec<models::HostedEnvironmentFileParam>**](HostedEnvironmentFileParam.md)> | Files available before the agent starts. Defaults to an empty list. | [optional]
**environment_template_id** | Option<**String**> | A reusable hosted template applied before inline configuration. Omitted fields inherit the template; network overrides cannot broaden its policy. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


