# BetaInputMessageResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> | The agent that produced this item. | [optional]
**r#type** | Option<**Type**> | The type of the message input. Always set to `message`.  (enum: message) | [optional]
**role** | **Role** | The role of the message input. One of `user`, `system`, or `developer`.  (enum: user, system, developer) | 
**status** | Option<**Status**> | The status of item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.  (enum: in_progress, completed, incomplete) | [optional]
**content** | [**Vec<models::BetaInputContent>**](BetaInputContent.md) | A list of one or many input items to the model, containing different content  types.  | 
**id** | **String** | The unique ID of the message input.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


