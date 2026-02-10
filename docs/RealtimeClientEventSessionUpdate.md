# RealtimeClientEventSessionUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | Option<**String**> | Optional client-generated ID used to identify this event. This is an arbitrary string that a client may assign. It will be passed back if there is an error with the event, but the corresponding `session.updated` event will not include it. | [optional]
**r#type** | **String** | The event type, must be `session.update`. | 
**session** | [**models::RealtimeClientEventSessionUpdateSession**](RealtimeClientEventSessionUpdate_session.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


