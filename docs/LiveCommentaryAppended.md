# LiveCommentaryAppended

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the Live server event. | 
**client_event_id** | Option<**String**> | The event_id of the client command associated with this server event, when supplied. | [optional]
**start_ms** | **i32** | The start of this event on the Live session timeline, in milliseconds from the beginning of the session. | 
**end_ms** | **i32** | The end of this event on the Live session timeline, in milliseconds from the beginning of the session. For appended context, this can equal start_ms. | 
**r#type** | **String** | The event type, always `session.commentary.appended`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


