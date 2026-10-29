# PublicEnvironmentResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::EnvironmentTypeResource**](EnvironmentTypeResource.md) |  | 
**id** | **String** | The ID of the environment. | 
**object** | **Object** | The object type. Always `agent.environment`. (enum: agent.environment) | 
**status** | [**models::EnvironmentStatusResource**](EnvironmentStatusResource.md) |  | 
**files** | [**Vec<models::HostedEnvironmentFileResource>**](HostedEnvironmentFileResource.md) | Files installed in the environment, without their contents. | 
**skills** | [**Vec<models::HostedSkillResource>**](HostedSkillResource.md) | Skills installed in the environment, without their archive contents. | 
**plugins** | [**Vec<models::HostedPluginResource>**](HostedPluginResource.md) | Plugins installed in the environment, without their archive contents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


