# ContainerAutoParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | Automatically creates a container for this request (enum: container_auto) | 
**file_ids** | Option<**Vec<String>**> | An optional list of uploaded files to make available to your code. | [optional]
**memory_limit** | Option<[**models::ContainerMemoryLimit**](ContainerMemoryLimit.md)> |  | [optional]
**network_policy** | Option<[**models::CreateContainerBodyNetworkPolicy**](CreateContainerBodyNetworkPolicy.md)> |  | [optional]
**skills** | Option<[**Vec<models::CreateContainerBodySkillsInner>**](CreateContainerBodySkillsInner.md)> | An optional list of skills referenced by id or inline data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


