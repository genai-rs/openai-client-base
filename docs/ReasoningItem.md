# ReasoningItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `reasoning`.  | 
**id** | **String** | The unique identifier of the reasoning content.  | 
**encrypted_content** | Option<**String**> | The encrypted content of the reasoning item - populated when a response is generated with `reasoning.encrypted_content` in the `include` parameter.  | [optional]
**summary** | [**Vec<models::Summary>**](Summary.md) | Reasoning summary content.  | 
**content** | Option<[**Vec<models::ReasoningTextContent>**](ReasoningTextContent.md)> | Reasoning text content.  | [optional]
**status** | Option<**String**> | The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


