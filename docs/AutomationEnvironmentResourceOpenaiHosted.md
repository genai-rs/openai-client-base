# AutomationEnvironmentResourceOpenaiHosted

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the object. Always `openai_hosted`.  Defaults to `\"openai_hosted\"`. (enum: openai_hosted) | 
**environment_id** | Option<**String**> | An existing environment reference, or null when provisioning a new environment. | 
**environment_template_id** | Option<**String**> | A template reference, or null when no template is selected. | 
**container_size** | Option<[**models::ContainerSizeResource**](ContainerSizeResource.md)> | The configured container size, or null to use the creation default. | 
**packages** | Option<[**models::EnvironmentPackagesResource**](EnvironmentPackagesResource.md)> | Package overrides, or null to inherit the template or creation default. | 
**network** | Option<[**models::NetworkPolicyResource**](NetworkPolicyResource.md)> | Network overrides, or null to inherit the template or creation default. | 
**desktop** | Option<[**models::DesktopResource**](DesktopResource.md)> | Desktop overrides, or null to inherit the template or creation default. | 
**capability_directories** | Option<**Vec<String>**> | Directory overrides, or null to inherit the template or creation default. | 
**skills** | Option<[**Vec<models::HostedTemplateSkillResource>**](HostedTemplateSkillResource.md)> | Skill overrides, excluding archive contents, or null to inherit. | 
**plugins** | Option<[**Vec<models::HostedPluginResource>**](HostedPluginResource.md)> | Plugin overrides, excluding archive contents, or null to inherit. | 
**files** | Option<[**Vec<models::HostedTemplateFileResource>**](HostedTemplateFileResource.md)> | File overrides, excluding contents, or null to inherit. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


