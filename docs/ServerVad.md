# ServerVad

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of turn detection, `server_vad` to turn on simple Server VAD.  | 
**threshold** | Option<**f64**> | Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A higher threshold will require louder audio to activate the model, and thus might perform better in noisy environments.  | [optional]
**prefix_padding_ms** | Option<**i32**> | Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in milliseconds). Defaults to 300ms.  | [optional]
**silence_duration_ms** | Option<**i32**> | Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults to 500ms. With shorter values the model will respond more quickly, but may jump in on short pauses from the user.  | [optional]
**create_response** | Option<**bool**> | Whether or not to automatically generate a response when a VAD stop event occurs.  | [optional]
**interrupt_response** | Option<**bool**> | Whether or not to automatically interrupt any ongoing response with output to the default conversation (i.e. `conversation` of `auto`) when a VAD start event occurs.  | [optional]
**idle_timeout_ms** | Option<**i32**> | Optional timeout after which a model response will be triggered automatically. This is useful for situations in which a long pause from the user is unexpected, such as a phone call. The model will effectively prompt the user to continue the conversation based on the current context.  The timeout value will be applied after the last model response's audio has finished playing, i.e. it's set to the `response.done` time plus audio playback duration.  An `input_audio_buffer.timeout_triggered` event (plus events associated with the Response) will be emitted when the timeout is reached. Idle timeout is currently only supported for `server_vad` mode.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


