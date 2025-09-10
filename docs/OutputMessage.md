# OutputMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique ID of the output message.  | 
**r#type** | **String** | The type of the output message. Always `message`.  | 
**role** | **String** | The role of the output message. Always `assistant`.  | 
**content** | [**Vec<models::OutputContent>**](OutputContent.md) | The content of the output message.  | 
**status** | **String** | The status of the message input. One of `in_progress`, `completed`, or `incomplete`. Populated when input items are returned via API.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


