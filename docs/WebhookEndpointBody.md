# WebhookEndpointBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique ID of the webhook endpoint. | 
**object** | **String** | The object type, which is always webhook_endpoint. | 
**created_at** | **i32** | The Unix timestamp when the endpoint was created. | 
**updated_at** | Option<**i32**> | The Unix timestamp of the last endpoint configuration or signing-secret change. Initialized at creation; tests and unchanged updates do not advance it. | [optional]
**name** | **String** | The human-readable name of the endpoint. | 
**url** | **String** | The HTTPS URL that receives webhook deliveries. | 
**event_types** | **Vec<String>** | The event types that trigger deliveries to this endpoint. | 
**signing_secret_hint** | Option<**String**> | A masked hint for the endpoint's signing secret. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


