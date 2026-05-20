# RealtimeTranslationSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the session that looks like `sess_1234567890abcdef`.  | 
**r#type** | **String** | The session type. Always `translation` for Realtime translation sessions.  | 
**expires_at** | **i32** | Expiration timestamp for the session, in seconds since epoch. | 
**model** | **String** | The Realtime translation model used for this session. This field is set at session creation and cannot be changed with `session.update`.  | 
**audio** | [**models::RealtimeTranslationSessionAudio**](RealtimeTranslationSession_audio.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


