# Filters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`. - `eq`: equals - `ne`: not equal - `gt`: greater than - `gte`: greater than or equal - `lt`: less than - `lte`: less than or equal  | 
**key** | **String** | The key to compare against the value. | 
**value** | [**models::ComparisonFilterValue**](ComparisonFilter_value.md) |  | 
**filters** | [**Vec<models::ComparisonFilter>**](ComparisonFilter.md) | Array of filters to combine. Items can be `ComparisonFilter` or `CompoundFilter`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


