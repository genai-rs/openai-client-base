# BetaPromptCacheMissDiagnosticsBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**reason** | [**models::BetaCacheMissReasonTypeEnum**](BetaCacheMissReasonTypeEnum.md) |  | 
**cache_missed_tokens** | **i32** | The estimated number of input tokens affected after the first detected divergence. | 
**comparison_reusable_tokens** | Option<**i32**> | The raw token count of the reusable prefix in the compared response. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


