# EnvironmentTemplateResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the reusable environment template. | 
**name** | Option<**String**> | An optional human-readable display name for the template. | 
**object** | **String** | The object type. Always `agent.environment.template`. | 
**created_at** | **i64** | The Unix timestamp, in seconds, when the template was created. | 
**updated_at** | **i64** | The Unix timestamp, in seconds, when the template was last updated. | 
**packages** | [**models::EnvironmentPackagesResource**](EnvironmentPackagesResource.md) |  | 
**network** | [**models::NetworkPolicyResource**](NetworkPolicyResource.md) |  | 
**capability_directories** | **Vec<String>** | Directories that expose capabilities to the agent. | 
**skills** | [**Vec<models::HostedTemplateSkillResource>**](HostedTemplateSkillResource.md) | Safe skill metadata, preserving unresolved version selectors. | 
**plugins** | [**Vec<models::HostedPluginResourceInline>**](HostedPluginResourceInline.md) | Safe plugin metadata, excluding inline archive contents. | 
**files** | [**Vec<models::HostedTemplateFileResource>**](HostedTemplateFileResource.md) | Safe file metadata, excluding contents and session-scoped file IDs. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


