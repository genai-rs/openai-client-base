# PublicEnvironmentResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the environment. | 
**object** | **String** | The object type. Always `agent.environment`. | 
**r#type** | [**models::EnvironmentTypeResource**](EnvironmentTypeResource.md) |  | 
**status** | [**models::EnvironmentStatusResource**](EnvironmentStatusResource.md) |  | 
**plugins** | [**Vec<models::HostedPluginResourceInline>**](HostedPluginResourceInline.md) | Plugins installed in the environment, without their archive contents. | 
**skills** | [**Vec<models::HostedSkillResource>**](HostedSkillResource.md) | Skills installed in the environment, without their archive contents. | 
**files** | [**Vec<models::HostedEnvironmentFileResource>**](HostedEnvironmentFileResource.md) | Files installed in the environment, without their contents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


