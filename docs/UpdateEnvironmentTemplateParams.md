# UpdateEnvironmentTemplateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | A replacement human-readable display name, or `null` to clear the name. | [optional]
**packages** | Option<[**models::EnvironmentPackagesParam**](EnvironmentPackagesParam.md)> |  | [optional]
**setup_commands** | Option<[**Vec<models::SetupCommandParam>**](SetupCommandParam.md)> | Replacement confidential setup commands, never included in returned resources. | [optional]
**network** | Option<[**models::NetworkPolicyParam**](NetworkPolicyParam.md)> |  | [optional]
**env** | Option<**std::collections::HashMap<String, String>**> | Replacement confidential environment values. | [optional]
**capability_directories** | Option<**Vec<String>**> | Directories that expose capabilities to the agent. | [optional]
**skills** | Option<[**Vec<models::HostedSkillParam>**](HostedSkillParam.md)> | Replacement skill configuration installed for each new session. | [optional]
**plugins** | Option<[**Vec<models::HostedPluginParamInline>**](HostedPluginParamInline.md)> | Replacement plugin configuration installed for each new session. | [optional]
**files** | Option<[**Vec<models::HostedEnvironmentFileParam>**](HostedEnvironmentFileParam.md)> | Replacement file configuration materialized for each new session. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


