# Content

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the input item. Always `input_audio`.  | 
**text** | **String** | The reasoning text from the model. | 
**image_url** | Option<**String**> | The URL of the image to be sent to the model. A fully qualified URL or base64 encoded image in a data URL. | [optional]
**file_id** | Option<**String**> | The ID of the file to be sent to the model. | [optional]
**detail** | [**models::ImageDetail**](ImageDetail.md) |  | 
**filename** | Option<**String**> | The name of the file to be sent to the model. | [optional]
**file_url** | Option<**String**> | The URL of the file to be sent to the model. | [optional]
**file_data** | Option<**String**> | The content of the file to be sent to the model.  | [optional]
**input_audio** | [**models::InputAudioInputAudio**](InputAudio_input_audio.md) |  | 
**annotations** | [**Vec<models::Annotation>**](Annotation.md) | The annotations of the text output. | 
**logprobs** | Option<[**Vec<models::LogProb>**](LogProb.md)> |  | [optional]
**refusal** | **String** | The refusal explanation from the model. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


