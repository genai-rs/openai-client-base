# VideoResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the video job. | 
**object** | **String** | The object type, which is always `video`. | 
**model** | [**models::VideoModel**](VideoModel.md) |  | 
**status** | [**models::VideoStatus**](VideoStatus.md) |  | 
**progress** | **i32** | Approximate completion percentage for the generation task. | 
**created_at** | **i32** | Unix timestamp (seconds) for when the job was created. | 
**completed_at** | Option<**i32**> | Unix timestamp (seconds) for when the job completed, if finished. | 
**expires_at** | Option<**i32**> | Unix timestamp (seconds) for when the downloadable assets expire, if set. | 
**size** | [**models::VideoSize**](VideoSize.md) |  | 
**seconds** | [**models::VideoSeconds**](VideoSeconds.md) |  | 
**remixed_from_video_id** | Option<**String**> | Identifier of the source video if this video is a remix. | 
**error** | Option<[**models::Error2**](Error2.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


