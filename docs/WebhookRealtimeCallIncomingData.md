# WebhookRealtimeCallIncomingData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**call_id** | **String** | The ID of the pending SIP call. Pass this value unchanged when accepting or rejecting the call through the Realtime API. For the Live API, use the `session_id` from `live.transport.incoming` instead.  | 
**sip_media_security** | Option<[**models::WebhookLiveCallIncomingDataSipMediaSecurity**](WebhookLiveCallIncoming_data_sip_media_security.md)> |  | [optional]
**sip_headers** | [**Vec<models::WebhookLiveCallIncomingDataSipHeadersInner>**](WebhookLiveCallIncoming_data_sip_headers_inner.md) | Headers from the SIP INVITE, excluding SIP authorization headers. Retained names, values, repeated entries, and order are preserved. Treat these values as untrusted call metadata.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


