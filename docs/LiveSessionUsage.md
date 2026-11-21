# LiveSessionUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seconds** | **i64** | The cumulative billable duration in whole seconds. When present, the active and paused breakdown adds up to this total. | 
**active_seconds** | Option<**i64**> | The remaining billable seconds attributed to active audio and any unused prepaid duration. | [optional]
**paused_seconds** | Option<**i64**> | Billable time attributed to paused inference at the configured rate, rounded down to whole seconds. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


