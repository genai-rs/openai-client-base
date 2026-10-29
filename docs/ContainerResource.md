# ContainerResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the container. | 
**object** | **String** | The type of this object. | 
**name** | **String** | Name of the container. | 
**created_at** | **i32** | Unix timestamp (in seconds) when the container was created. | 
**status** | **String** | Status of the container (e.g., active, deleted). | 
**last_active_at** | Option<**i32**> | Unix timestamp (in seconds) when the container was last active. | [optional]
**expires_after** | Option<[**models::ContainerResourceExpiresAfter**](ContainerResourceExpiresAfter.md)> |  | [optional]
**memory_limit** | Option<**MemoryLimit**> | The memory limit configured for the container. (enum: 1g, 4g, 16g, 64g) | [optional]
**network_policy** | Option<[**models::ContainerResourceNetworkPolicy**](ContainerResourceNetworkPolicy.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


