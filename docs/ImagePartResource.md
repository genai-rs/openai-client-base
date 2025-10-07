# ImagePartResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the uploaded image. | 
**r#type** | **String** | Type discriminator that is always `image`. | 
**mime_type** | **String** | MIME type of the uploaded image. | 
**name** | Option<**String**> | Original filename for the uploaded image. Defaults to null when unnamed. | 
**preview_url** | **String** | Preview URL that can be rendered inline for the image. | 
**upload_url** | Option<**String**> | Signed URL for downloading the uploaded image. Defaults to null when no download link is available. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


