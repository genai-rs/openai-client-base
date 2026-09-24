# BetaErrorPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The error type that was emitted. | 
**code** | Option<**String**> | The error code that was emitted, if any. | 
**message** | **String** | The human-readable error message that was emitted. | 
**param** | Option<**String**> | The parameter name that was associated with the error, if any. | 
**headers** | Option<**std::collections::HashMap<String, String>**> | The response headers that were emitted with the error, if any. | [optional]
**misalignment** | Option<[**models::BetaMisalignmentErrorDetailsResource**](BetaMisalignmentErrorDetailsResource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


