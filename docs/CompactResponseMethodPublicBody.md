# CompactResponseMethodPublicBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<[**models::ModelIdsCompaction**](ModelIdsCompaction.md)> |  | 
**input** | Option<[**models::TokenCountsBodyInput**](TokenCountsBody_input.md)> |  | [optional]
**previous_response_id** | Option<**String**> | The unique ID of the previous response to the model. Use this to create multi-turn conversations. Learn more about [conversation state](/docs/guides/conversation-state). Cannot be used in conjunction with `conversation`. | [optional]
**instructions** | Option<**String**> | A system (or developer) message inserted into the model's context. When used along with `previous_response_id`, the instructions from a previous response will not be carried over to the next response. This makes it simple to swap out system (or developer) messages in new responses. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


