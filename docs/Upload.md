# Upload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The Upload unique identifier, which can be referenced in API endpoints. | 
**created_at** | **i32** | The Unix timestamp (in seconds) for when the Upload was created. | 
**filename** | **String** | The name of the file to be uploaded. | 
**bytes** | **i32** | The intended number of bytes to be uploaded. | 
**purpose** | **String** | The intended purpose of the file. [Please refer here](https://developers.openai.com/api/reference/resources/files#%28resource%29%20files%20%3E%20%28model%29%20file_object%20%3E%20%28schema%29%20%3E%20%28property%29%20purpose) for acceptable values. | 
**status** | **String** | The status of the Upload. | 
**expires_at** | **i32** | The Unix timestamp (in seconds) for when the Upload will expire. | 
**object** | Option<**String**> | The object type, which is always \"upload\". | [optional]
**file** | Option<[**models::OpenAiFile**](OpenAIFile.md)> | The ready File object after the Upload is completed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


