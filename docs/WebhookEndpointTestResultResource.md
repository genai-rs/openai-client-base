# WebhookEndpointTestResultResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | The object type, which is always webhook_endpoint.test. | 
**webhook_endpoint_id** | **String** | The ID of the webhook endpoint that received the test. | 
**event_type** | **String** | The event type sent in the test. | 
**status_code** | **i32** | The HTTP status code returned by the endpoint. | 
**success** | **bool** | Whether the test request completed. Always true for returned results; use status_code to determine the endpoint response. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


