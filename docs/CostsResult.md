# CostsResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_source** | Option<**ApiSource**> | When grouped by `api_source`, `agents_api` identifies attributed Agents API activity and `unlabeled` includes all records without published source attribution, including historical and unknown origins. Unlabeled does not imply direct API usage. Without source grouping, this field is null. (enum: agents_api, unlabeled) | [optional]
**object** | **Object** |  (enum: organization.costs.result) | 
**amount** | Option<[**models::CostsResultAmount**](CostsResultAmount.md)> |  | [optional]
**line_item** | Option<**String**> | When `group_by=line_item`, this field provides the line item of the grouped costs result. | [optional]
**project_id** | Option<**String**> | When `group_by=project_id`, this field provides the project ID of the grouped costs result. | [optional]
**user_id** | Option<**String**> | When `group_by=user_id`, this field provides the user ID of the grouped costs result. | [optional]
**api_key_id** | Option<**String**> | When `group_by=api_key_id`, this field provides the API Key ID of the grouped costs result. | [optional]
**quantity** | Option<**f64**> | When `group_by=line_item`, this field provides the quantity of the grouped costs result. | [optional]
**quantity_unit** | Option<[**models::CostsResultQuantityUnit**](CostsResultQuantityUnit.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


