# BetaReasoningItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**r#type** | **String** | The type of the object. Always `reasoning`.  | 
**id** | **String** | The unique identifier of the reasoning content.  | 
**encrypted_content** | Option<**String**> | The encrypted content of the reasoning item. This is populated by default for reasoning items returned by `POST /v1/responses` and WebSocket `response.create` requests.  | [optional]
**summary** | [**Vec<models::BetaSummaryTextContent>**](BetaSummaryTextContent.md) | Reasoning summary content.  | 
**content** | Option<[**Vec<models::BetaReasoningTextContent>**](BetaReasoningTextContent.md)> | Reasoning text content.  | [optional]
**status** | Option<**String**> | The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


