# ClientSecretExpiration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anchor** | Option<**String**> | The anchor point for the client secret expiration, meaning that `seconds` will be added to the `created_at` time of the client secret to produce an expiration timestamp. Only `created_at` is currently supported.  | [optional]
**seconds** | Option<**i32**> | The number of seconds from the anchor point to the expiration. Select a value between `10` and `7200` (2 hours). This default to 600 seconds (10 minutes) if not specified.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


