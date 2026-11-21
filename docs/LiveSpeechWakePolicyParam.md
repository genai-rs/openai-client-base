# LiveSpeechWakePolicyParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The wake policy type, always `speech`.  Defaults to `\"speech\"`. (enum: speech) | 
**vad_threshold** | Option<**f64**> | The speech detection threshold for the first wake and each later wake, from 0 to 1 inclusive. Defaults to 0.5. Higher values require stronger evidence of speech. This setting is independent of active turn detection. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


