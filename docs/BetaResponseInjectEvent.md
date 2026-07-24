# BetaResponseInjectEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The event discriminator. Always `response.inject`. | 
**response_id** | **String** | The ID of the active response that should receive the input. | 
**input** | [**Vec<models::BetaInputItem>**](BetaInputItem.md) | Input items to inject into the active response. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


