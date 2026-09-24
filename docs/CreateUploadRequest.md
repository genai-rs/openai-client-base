# CreateUploadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filename** | **String** | The name of the file to upload.  | 
**purpose** | **String** | The intended purpose of the uploaded file.  See the [documentation on File purposes](https://developers.openai.com/api/reference/resources/files/methods/create#%28resource%29%20files%20%3E%20%28method%29%20create%20%3E%20%28params%29%200%20%3E%20%28param%29%20purpose%20%3E%20%28schema%29).  | 
**bytes** | **i32** | The number of bytes in the file you are uploading.  | 
**mime_type** | **String** | The MIME type of the file.   This must fall within the supported MIME types for your file purpose. See the supported MIME types for assistants and vision.  | 
**expires_after** | Option<[**models::FileExpirationAfter**](FileExpirationAfter.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


