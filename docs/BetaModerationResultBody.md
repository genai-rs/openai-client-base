# BetaModerationResultBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The object type, which was always `moderation_result` for successful moderation results. | 
**model** | **String** | The moderation model that produced this result. | 
**flagged** | **bool** | A boolean indicating whether the content was flagged by any category. | 
**categories** | **std::collections::HashMap<String, bool>** | A dictionary of moderation categories to booleans, True if the input is flagged under this category. | 
**category_scores** | **std::collections::HashMap<String, f64>** | A dictionary of moderation categories to scores. | 
**category_applied_input_types** | [**std::collections::HashMap<String, Vec<models::BetaModerationInputType>>**](Vec.md) | Which modalities of input are reflected by the score for each category. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


