# ContainerAutoParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Automatically creates a container for this request | 
**file_ids** | Option<**Vec<String>**> | An optional list of uploaded files to make available to your code. | [optional]
**memory_limit** | Option<[**models::ContainerMemoryLimit**](ContainerMemoryLimit.md)> |  | [optional]
**network_policy** | Option<[**models::AutoCodeInterpreterToolParamNetworkPolicy**](AutoCodeInterpreterToolParam_network_policy.md)> |  | [optional]
**skills** | Option<[**Vec<models::ContainerAutoParamSkillsInner>**](ContainerAutoParam_skills_inner.md)> | An optional list of skills referenced by id or inline data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


