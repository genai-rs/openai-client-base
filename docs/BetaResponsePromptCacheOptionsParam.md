# BetaResponsePromptCacheOptionsParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ttl** | Option<**Ttl**> | The minimum lifetime applied to every implicit and explicit cache breakpoint written by the request. Supported values are `30m` (the default) and `12h`. The backend may retain cache entries for longer. (enum: 30m, 12h) | [optional]
**mode** | Option<[**models::BetaPromptCacheModeEnum**](BetaPromptCacheModeEnum.md)> |  | [optional]
**prewarm** | Option<**bool**> | Prepares the prompt cache without generating output. Defaults to `false`. When set to `true`, overrides the `generate` field to `false`. | [optional]
**comparison_response_id** | Option<**String**> | The ID of a response to compare when diagnosing prompt cache reuse. Supplying this field requests prompt cache diagnostics when the feature is enabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


