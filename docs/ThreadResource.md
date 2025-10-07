# ThreadResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Identifier of the thread. | 
**object** | **String** | Type discriminator that is always `chatkit.thread`. | 
**created_at** | **i32** | Unix timestamp (in seconds) for when the thread was created. | 
**title** | Option<**String**> | Optional human-readable title for the thread. Defaults to null when no title has been generated. | 
**status** | [**models::ThreadResourceStatus**](ThreadResource_status.md) |  | 
**user** | **String** | Free-form string that identifies your end user who owns the thread. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


