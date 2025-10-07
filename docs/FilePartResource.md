# FilePartResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the uploaded file. | 
**r#type** | **String** | Type discriminator that is always `file`. | 
**name** | Option<**String**> | Original filename supplied by the uploader. Defaults to null when unnamed. | 
**mime_type** | Option<**String**> | MIME type reported for the uploaded file. Defaults to null when unknown. | 
**upload_url** | Option<**String**> | Signed URL for downloading the uploaded file. Defaults to null when no download link is available. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


