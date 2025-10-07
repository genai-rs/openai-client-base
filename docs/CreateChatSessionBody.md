# CreateChatSessionBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow** | [**models::WorkflowParam**](WorkflowParam.md) |  | 
**user** | **String** | A free-form string that identifies your end user; ensures this Session can access other objects that have the same `user` scope. | 
**expires_after** | Option<[**models::ExpiresAfterParam**](ExpiresAfterParam.md)> |  | [optional]
**rate_limits** | Option<[**models::RateLimitsParam**](RateLimitsParam.md)> |  | [optional]
**chatkit_configuration** | Option<[**models::ChatkitConfigurationParam**](ChatkitConfigurationParam.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


