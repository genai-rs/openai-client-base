# LiveHostedShellContainerAutoParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | Automatically creates a container for this request  Defaults to `\"container_auto\"`. (enum: container_auto) | 
**file_ids** | Option<**Vec<String>**> | An optional list of uploaded files to make available to your code. | [optional]
**memory_limit** | Option<[**models::LiveContainerMemoryLimit**](LiveContainerMemoryLimit.md)> |  | [optional]
**network_policy** | Option<[**models::LiveHostedShellContainerAutoParamNetworkPolicy**](LiveHostedShellContainerAutoParamNetworkPolicy.md)> |  | [optional]
**skills** | Option<[**Vec<models::LiveHostedShellContainerAutoParamSkillsInner>**](LiveHostedShellContainerAutoParamSkillsInner.md)> | An optional list of skills referenced by id or inline data. Across all inline skills in this list, decoded ZIP data must total at most 50 MiB and Base64 text must total at most 70,254,592 characters, including line breaks. Skill references do not count toward these limits. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


