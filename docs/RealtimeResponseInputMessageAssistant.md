# RealtimeResponseInputMessageAssistant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | An optional client ID. The value `root` is reserved. | [optional]
**r#type** | **Type** |  (enum: message) | 
**status** | Option<**Status**> | Optional input status. Omission or null defaults to completed.  Defaults to `\"completed\"`. (enum: completed, incomplete, ) | [optional]
**role** | **Role** |  (enum: assistant) | 
**content** | [**Vec<models::RealtimeResponseInputAssistantText>**](RealtimeResponseInputAssistantText.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


