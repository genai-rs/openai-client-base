# DecisionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | **String** | The model to use. Only `gpt-6-luna` is supported. | 
**input** | [**models::DecisionInput**](DecisionInput.md) |  | 
**questions** | [**Vec<models::QuestionParam>**](QuestionParam.md) | Ask 1–200 questions about the input. Each question can be a predicate, choice, or score. Answers come back in the same order. | 
**safety_identifier** | Option<**String**> | Opaque caller-provided end-user identifier, scoped by the verified org. Match Responses' limit; this is never the authenticated user identity. | [optional]
**prompt_cache_key** | Option<**String**> | Partition prompt cache reuse by caller-provided key. | [optional]
**prompt_cache_options** | Option<[**models::PromptCacheOptions**](Prompt_cache_options.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


