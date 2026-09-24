# LiveSessionClosed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the Live server event. | 
**client_event_id** | Option<**String**> | The event_id of the client command associated with this server event, when supplied. | [optional]
**r#type** | **String** | The event type, always `session.closed`. | 
**reason** | [**models::LiveSessionClosedReason**](LiveSessionClosed_reason.md) |  | 
**session** | [**models::LiveSessionResourceParam**](LiveSessionResourceParam.md) |  | 
**usage** | [**models::LiveSessionUsage**](LiveSessionUsage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


