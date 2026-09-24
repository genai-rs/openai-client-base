# EnvironmentResourceOpenaiHosted

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `openai_hosted`. | 
**id** | **String** | The public ID of the environment. | 
**packages** | [**models::EnvironmentPackagesResource**](EnvironmentPackagesResource.md) |  | 
**network** | [**models::NetworkPolicyResource**](NetworkPolicyResource.md) |  | 
**capability_directories** | **Vec<String>** | Directories that contain capabilities exposed to the agent. | 
**skills** | [**Vec<models::HostedSkillResource>**](HostedSkillResource.md) | Skills installed in the environment, excluding their archive contents. | 
**plugins** | [**Vec<models::HostedPluginResourceInline>**](HostedPluginResourceInline.md) | Plugins installed in the environment, excluding their archive contents. | 
**files** | [**Vec<models::HostedEnvironmentFileResource>**](HostedEnvironmentFileResource.md) | Files available in the environment, excluding their contents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


