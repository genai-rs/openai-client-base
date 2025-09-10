# BatchFileExpirationAfter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anchor** | **String** | Anchor timestamp after which the expiration policy applies. Supported anchors: `created_at`. Note that the anchor is the file creation time, not the time the batch is created. | 
**seconds** | **i32** | The number of seconds after the anchor time that the file will expire. Must be between 3600 (1 hour) and 2592000 (30 days). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


