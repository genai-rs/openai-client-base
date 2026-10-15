# Error2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | A machine-readable error code that was returned. | 
**r#type** | Option<**String**> | The original error type, or null if it was not recorded. | [optional]
**param** | Option<**String**> | The request parameter associated with the error, or null if none was recorded. | [optional]
**message** | **String** | A human-readable description of the error that was returned. | 
**headers** | Option<**std::collections::HashMap<String, String>**> | The Retry-After and Retry-After-Ms headers returned with the original error, if any. | [optional]
**misalignment** | Option<[**models::MisalignmentErrorDetailsResource**](MisalignmentErrorDetailsResource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


