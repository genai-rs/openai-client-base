# UsageWebSearchCallsResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_source** | Option<**ApiSource**> | When grouped by `api_source`, `agents_api` identifies attributed Agents API activity and `unlabeled` includes all records without published source attribution, including historical and unknown origins. Unlabeled does not imply direct API usage. Without source grouping, this field is null. (enum: agents_api, unlabeled) | [optional]
**object** | **Object** |  (enum: organization.usage.web_searches.result) | 
**num_model_requests** | **i32** | The count of model requests. | 
**num_requests** | **i32** | The count of web search calls. | 
**project_id** | Option<**String**> | When `group_by=project_id`, this field provides the project ID of the grouped usage result. | [optional]
**user_id** | Option<**String**> | When `group_by=user_id`, this field provides the user ID of the grouped usage result. | [optional]
**api_key_id** | Option<**String**> | When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result. | [optional]
**model** | Option<**String**> | ID of the model to use | [optional]
**context_level** | Option<**String**> | When `group_by=context_level`, this field provides the search context size of the grouped usage result. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


