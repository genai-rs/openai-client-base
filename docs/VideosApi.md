# \VideosApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_video**](VideosApi.md#create_video) | **POST** /videos | Create a new video generation job from a prompt and optional reference assets.
[**create_video_remix**](VideosApi.md#create_video_remix) | **POST** /videos/{video_id}/remix | Create a remix of a completed video using a refreshed prompt.
[**delete_video**](VideosApi.md#delete_video) | **DELETE** /videos/{video_id} | Permanently delete a completed or failed video and its stored assets.
[**get_video**](VideosApi.md#get_video) | **GET** /videos/{video_id} | Fetch the latest metadata for a generated video.
[**list_videos**](VideosApi.md#list_videos) | **GET** /videos | List recently generated videos for the current project.
[**retrieve_video_content**](VideosApi.md#retrieve_video_content) | **GET** /videos/{video_id}/content | Download the generated video bytes or a derived preview asset.  Streams the rendered video content for the specified video job.



## create_video

> models::VideoResource create_video(prompt, model, input_reference, seconds, size)
Create a new video generation job from a prompt and optional reference assets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt** | **String** | Text prompt that describes the video to generate. | [required] |
**model** | Option<[**models::VideoModel**](VideoModel.md)> |  |  |
**input_reference** | Option<**std::path::PathBuf**> | Optional image reference that guides generation. |  |
**seconds** | Option<[**models::VideoSeconds**](VideoSeconds.md)> |  |  |
**size** | Option<[**models::VideoSize**](VideoSize.md)> |  |  |

### Return type

[**models::VideoResource**](VideoResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_video_remix

> models::VideoResource create_video_remix(video_id, prompt)
Create a remix of a completed video using a refreshed prompt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | The identifier of the completed video to remix. | [required] |
**prompt** | **String** | Updated text prompt that directs the remix generation. | [required] |

### Return type

[**models::VideoResource**](VideoResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_video

> models::DeletedVideoResource delete_video(video_id)
Permanently delete a completed or failed video and its stored assets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | The identifier of the video to delete. | [required] |

### Return type

[**models::DeletedVideoResource**](DeletedVideoResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_video

> models::VideoResource get_video(video_id)
Fetch the latest metadata for a generated video.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | The identifier of the video to retrieve. | [required] |

### Return type

[**models::VideoResource**](VideoResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_videos

> models::VideoListResource list_videos(limit, order, after)
List recently generated videos for the current project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of items to retrieve |  |
**order** | Option<[**OrderEnum**](.md)> | Sort order of results by timestamp. Use `asc` for ascending order or `desc` for descending order. |  |
**after** | Option<**String**> | Identifier for the last item from the previous pagination request |  |

### Return type

[**models::VideoListResource**](VideoListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_video_content

> std::path::PathBuf retrieve_video_content(video_id, variant)
Download the generated video bytes or a derived preview asset.  Streams the rendered video content for the specified video job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | The identifier of the video whose media to download. | [required] |
**variant** | Option<[**VideoContentVariant**](.md)> | Which downloadable asset to return. Defaults to the MP4 video. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: video/mp4, image/webp, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

