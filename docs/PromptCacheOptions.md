# PromptCacheOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ttl** | Option<**String**> | The minimum lifetime applied to every implicit and explicit cache breakpoint written by the request. Defaults to `30m`, which is currently the only supported value. The backend may retain cache entries for longer. | [optional]
**mode** | Option<**String**> | Controls whether OpenAI automatically creates an implicit cache breakpoint. Defaults to `implicit`. With `implicit`, OpenAI creates one implicit breakpoint and writes up to the latest three explicit breakpoints in the request. With `explicit`, OpenAI does not create an implicit breakpoint and writes up to the latest four explicit breakpoints. If there are no explicit breakpoints, the request does not use prompt caching. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


