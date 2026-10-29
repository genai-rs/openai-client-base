# RetentionRatioTruncation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | Use retention ratio truncation. (enum: retention_ratio) | 
**retention_ratio** | **f64** | Fraction of post-instruction conversation tokens to retain (`0.0` - `1.0`) when the conversation exceeds the input token limit. Setting this to `0.8` means that messages will be dropped until 80% of the maximum allowed tokens are used. This helps reduce the frequency of truncations and improve cache rates.  | 
**token_limits** | Option<[**models::RetentionRatioTruncationTokenLimits**](RetentionRatioTruncationTokenLimits.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


