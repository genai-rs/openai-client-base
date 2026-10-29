# ResponseError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The original error type, or null if it was not recorded. | [optional]
**param** | Option<**String**> | The request parameter associated with the error, or null if none was recorded. | [optional]
**code** | [**models::ResponseErrorCode**](ResponseErrorCode.md) |  | 
**message** | **String** | A human-readable description of the error.  | 
**misalignment** | Option<[**models::MisalignmentErrorDetailsResource**](MisalignmentErrorDetailsResource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


