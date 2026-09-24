# WebhookLiveTransportIncomingData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The incoming transport type. Always `sip`. | 
**session_id** | **String** | The `live_...` ID of the pending SIP session. Forward this value unchanged when accepting or rejecting the call through the Live API.  | 
**sip_media_security** | Option<[**models::WebhookLiveCallIncomingDataSipMediaSecurity**](WebhookLiveCallIncoming_data_sip_media_security.md)> |  | [optional]
**sip_headers** | [**Vec<models::WebhookLiveCallIncomingDataSipHeadersInner>**](WebhookLiveCallIncoming_data_sip_headers_inner.md) | Headers from the SIP INVITE, excluding SIP authorization headers. Retained names, values, repeated entries, and order are preserved. Treat these values as untrusted call metadata.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


