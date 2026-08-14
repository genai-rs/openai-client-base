# UsageCompletionsResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** |  | 
**input_tokens** | **i32** | The aggregated number of input tokens used, including cached and cache-write tokens. This includes text, audio, and image tokens. For customers subscribed to Scale Tier, this includes Scale Tier tokens. | 
**input_cached_tokens** | Option<**i32**> | The aggregated number of cached input tokens used across text, audio, and image inputs. For customers subscribed to Scale Tier, this includes Scale Tier tokens. | [optional]
**input_cache_write_tokens** | Option<**i32**> | The aggregated number of input tokens written to the cache. | [optional]
**input_uncached_tokens** | Option<**i32**> | The aggregated number of uncached input tokens used across text, audio, and image inputs, excluding cache-write tokens. | [optional]
**output_tokens** | **i32** | The aggregated number of output tokens used across text, audio, and image outputs. For customers subscribed to Scale Tier, this includes Scale Tier tokens. | 
**input_text_tokens** | Option<**i32**> | The aggregated number of uncached text input tokens used, excluding cache-write tokens. | [optional]
**output_text_tokens** | Option<**i32**> | The aggregated number of text output tokens used. | [optional]
**input_cached_text_tokens** | Option<**i32**> | The aggregated number of cached text input tokens used. | [optional]
**input_audio_tokens** | Option<**i32**> | The aggregated number of uncached audio input tokens used. | [optional]
**input_cached_audio_tokens** | Option<**i32**> | The aggregated number of cached audio input tokens used. | [optional]
**output_audio_tokens** | Option<**i32**> | The aggregated number of audio output tokens used. | [optional]
**input_image_tokens** | Option<**i32**> | The aggregated number of uncached image input tokens used. | [optional]
**input_cached_image_tokens** | Option<**i32**> | The aggregated number of cached image input tokens used. | [optional]
**output_image_tokens** | Option<**i32**> | The aggregated number of image output tokens used. | [optional]
**num_model_requests** | **i32** | The count of requests made to the model. | 
**project_id** | Option<**String**> | When `group_by=project_id`, this field provides the project ID of the grouped usage result. | [optional]
**user_id** | Option<**String**> | When `group_by=user_id`, this field provides the user ID of the grouped usage result. | [optional]
**api_key_id** | Option<**String**> | When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result. | [optional]
**model** | Option<**String**> | ID of the model to use | [optional]
**batch** | Option<**bool**> | When `group_by=batch`, this field tells whether the grouped usage result is batch or not. | [optional]
**service_tier** | Option<**String**> | When `group_by=service_tier`, this field provides the service tier of the grouped usage result. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


