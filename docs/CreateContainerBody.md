# CreateContainerBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the container to create. | 
**file_ids** | Option<**Vec<String>**> | IDs of files to copy to the container. | [optional]
**expires_after** | Option<[**models::CreateContainerBodyExpiresAfter**](CreateContainerBody_expires_after.md)> |  | [optional]
**skills** | Option<[**Vec<models::CreateContainerBodySkillsInner>**](CreateContainerBody_skills_inner.md)> | An optional list of skills referenced by id or inline data. | [optional]
**memory_limit** | Option<**String**> | Optional memory limit for the container. Defaults to \"1g\". | [optional]
**network_policy** | Option<[**models::CreateContainerBodyNetworkPolicy**](CreateContainerBody_network_policy.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


