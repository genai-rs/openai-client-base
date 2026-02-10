# CreateTranscriptionRequestChunkingStrategy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Must be set to `server_vad` to enable manual chunking using server side VAD. | 
**prefix_padding_ms** | Option<**i32**> | Amount of audio to include before the VAD detected speech (in  milliseconds).  | [optional]
**silence_duration_ms** | Option<**i32**> | Duration of silence to detect speech stop (in milliseconds). With shorter values the model will respond more quickly,  but may jump in on short pauses from the user.  | [optional]
**threshold** | Option<**f64**> | Sensitivity threshold (0.0 to 1.0) for voice activity detection. A  higher threshold will require louder audio to activate the model, and  thus might perform better in noisy environments.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


