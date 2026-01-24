# CreateVideoBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<[**models::VideoModel**](VideoModel.md)> |  | [optional]
**prompt** | **String** | Text prompt that describes the video to generate. | 
**input_reference** | Option<[**std::path::PathBuf**](std::path::PathBuf.md)> | Optional image reference that guides generation. | [optional]
**seconds** | Option<[**models::VideoSeconds**](VideoSeconds.md)> |  | [optional]
**size** | Option<[**models::VideoSize**](VideoSize.md)> |  | [optional]
**character_ids** | Option<**Vec<String>**> | Character IDs to include in the generation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


