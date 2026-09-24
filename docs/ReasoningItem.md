# ReasoningItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `reasoning`.  | 
**id** | **String** | The unique identifier of the reasoning content.  | 
**encrypted_content** | Option<**String**> | The encrypted content of the reasoning item. This is populated by default for reasoning items returned by `POST /v1/responses` and WebSocket `response.create` requests.  When streaming, use the completed reasoning item and its `encrypted_content` from the `response.output_item.done` event in subsequent requests. The `encrypted_content` in `response.output_item.added` may be incomplete. This is especially important when `store` is `false` or when using Zero Data Retention.  | [optional]
**summary** | [**Vec<models::SummaryTextContent>**](SummaryTextContent.md) | Reasoning summary content.  | 
**content** | Option<[**Vec<models::ReasoningTextContent>**](ReasoningTextContent.md)> | Reasoning text content.  | [optional]
**status** | Option<**String**> | The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


