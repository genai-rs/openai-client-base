# InputMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The type of the message input. Always set to `message`.  | [optional]
**role** | **String** | The role of the message input. One of `user`, `system`, or `developer`.  | 
**status** | Option<**String**> | The status of item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.  | [optional]
**content** | [**Vec<models::InputContent>**](InputContent.md) | A list of one or many input items to the model, containing different content  types.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


