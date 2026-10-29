# BetaFilters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `in`, `nin`. - `eq`: equals - `ne`: not equal - `gt`: greater than - `gte`: greater than or equal - `lt`: less than - `lte`: less than or equal - `in`: in - `nin`: not in  (enum: eq, ne, gt, gte, lt, lte, in, nin, and, or) | 
**key** | **String** | The key to compare against the value. | 
**value** | [**models::BetaComparisonFilterValue**](BetaComparisonFilterValue.md) |  | 
**filters** | [**Vec<models::BetaCompoundFilterFiltersInner>**](BetaCompoundFilterFiltersInner.md) | Array of filters to combine. Items can be `ComparisonFilter` or `CompoundFilter`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


