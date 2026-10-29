# CreateContainerBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the container to create. | 
**file_ids** | Option<**Vec<String>**> | IDs of files to copy to the container. | [optional]
**expires_after** | Option<[**models::CreateContainerBodyExpiresAfter**](CreateContainerBodyExpiresAfter.md)> |  | [optional]
**skills** | Option<[**Vec<models::CreateContainerBodySkillsInner>**](CreateContainerBodySkillsInner.md)> | An optional list of skills referenced by id or inline data. | [optional]
**memory_limit** | Option<**MemoryLimit**> | Optional memory limit for the container. Defaults to \"1g\". (enum: 1g, 4g, 16g, 64g) | [optional]
**network_policy** | Option<[**models::CreateContainerBodyNetworkPolicy**](CreateContainerBodyNetworkPolicy.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


