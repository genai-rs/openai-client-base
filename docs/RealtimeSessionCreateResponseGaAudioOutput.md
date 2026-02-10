# RealtimeSessionCreateResponseGaAudioOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**format** | Option<[**models::RealtimeAudioFormats**](RealtimeAudioFormats.md)> |  | [optional]
**voice** | Option<**String**> | Model identifier as string | [optional]
**speed** | Option<**f64**> | The speed of the model's spoken response as a multiple of the original speed. 1.0 is the default speed. 0.25 is the minimum speed. 1.5 is the maximum speed. This value can only be changed in between model turns, not while a response is in progress.  This parameter is a post-processing adjustment to the audio after it is generated, it's also possible to prompt the model to speak faster or slower.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


