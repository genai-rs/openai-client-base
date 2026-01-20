# CreateUploadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filename** | **String** | The name of the file to upload.  | 
**purpose** | **String** | The intended purpose of the uploaded file.  See the [documentation on File purposes](https://platform.openai.com/docs/api-reference/files/create#files-create-purpose).  | 
**bytes** | **i32** | The number of bytes in the file you are uploading.  | 
**mime_type** | **String** | The MIME type of the file.   This must fall within the supported MIME types for your file purpose. See the supported MIME types for assistants and vision.  | 
**expires_after** | Option<[**models::FileExpirationAfter**](FileExpirationAfter.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


