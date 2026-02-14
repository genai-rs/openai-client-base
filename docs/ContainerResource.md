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
**expires_after** | Option<[**models::ContainerResourceExpiresAfter**](ContainerResource_expires_after.md)> |  | [optional]
**memory_limit** | Option<**String**> | The memory limit configured for the container. | [optional]
**network_policy** | Option<[**models::ContainerResourceNetworkPolicy**](ContainerResource_network_policy.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


