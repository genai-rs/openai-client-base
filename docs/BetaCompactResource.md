# BetaCompactResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image** | Option<[**models::BetaResolvedImageOptions**](BetaResolvedImageOptions.md)> |  | [optional]
**id** | **String** | The unique identifier for the compacted response. | 
**object** | **Object** | The object type. Always `response.compaction`.  Defaults to `\"response.compaction\"`. (enum: response.compaction) | 
**safety_identifier** | Option<**String**> | A stable identifier that was used for safety monitoring and abuse detection. | [optional]
**output** | [**Vec<models::BetaItemField>**](BetaItemField.md) | The compacted list of output items. | 
**created_at** | **jiff::Timestamp** | Unix timestamp (in seconds) when the compacted conversation was created. | 
**service_tier** | Option<**String**> | The service tier used for compaction, when reported. | [optional]
**usage** | [**models::BetaResponseUsage**](BetaResponseUsage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


