# SafetyAlertResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**object** | **String** |  | 
**created_at** | **i32** |  | 
**request_id** | **String** |  | 
**response_id** | **String** |  | 
**model** | **String** |  | 
**request_paused** | **bool** | Whether block registration succeeded for this request. This does not confirm that response execution stopped. | 
**error_type** | [**models::SafetyAlertErrorType**](SafetyAlertErrorType.md) |  | 
**reason** | Option<**String**> | A customer-safe description derived from error_type, or null for zero data retention requests. | 
**detailed_explanation** | Option<**String**> | A generated explanation, temporarily available for eligible zero data retention alerts. Omitted when unavailable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


