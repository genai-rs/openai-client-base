# ChatSessionResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Identifier for the ChatKit session. | 
**object** | **String** | Type discriminator that is always `chatkit.session`. | 
**expires_at** | **i32** | Unix timestamp (in seconds) for when the session expires. | 
**client_secret** | **String** | Ephemeral client secret that authenticates session requests. | 
**workflow** | [**models::ChatkitWorkflow**](ChatkitWorkflow.md) |  | 
**user** | **String** | User identifier associated with the session. | 
**rate_limits** | [**models::ChatSessionRateLimits**](ChatSessionRateLimits.md) |  | 
**max_requests_per_1_minute** | **i32** | Convenience copy of the per-minute request limit. | 
**status** | [**models::ChatSessionStatus**](ChatSessionStatus.md) |  | 
**chatkit_configuration** | [**models::ChatSessionChatkitConfiguration**](ChatSessionChatkitConfiguration.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


