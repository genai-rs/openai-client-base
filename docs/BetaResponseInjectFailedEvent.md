# BetaResponseInjectFailedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The event discriminator. Always `response.inject.failed`. | 
**response_id** | **String** | The ID of the response that rejected the input. | 
**input** | [**Vec<models::BetaInputItem>**](BetaInputItem.md) | The raw input items that were not committed. | 
**error** | [**models::BetaResponseInjectFailedEventError**](BetaResponseInjectFailedEvent_error.md) |  | 
**sequence_number** | **i32** | The sequence number for this event. | 
**stream_id** | Option<**String**> | The multiplexed WebSocket stream that emitted the event. This field is present only when WebSocket multiplexing is enabled separately.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


