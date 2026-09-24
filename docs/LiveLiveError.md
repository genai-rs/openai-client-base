# LiveLiveError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The category of error, such as `invalid_request_error` for an invalid Live client command. | 
**code** | **String** | A machine-readable code identifying the Live error, such as `unknown_parameter`. | 
**message** | **String** | A human-readable explanation of the Live error. | 
**param** | Option<**String**> | The parameter that caused the error, when applicable, such as `session.voice`. | [optional]
**client_event_id** | Option<**String**> | The event_id of the client command that caused the error, when supplied. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


