# BetaTokenCountsBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> | ID of the model to use | [optional]
**input** | Option<[**models::BetaTokenCountsBodyInput**](BetaTokenCountsBody_input.md)> |  | [optional]
**previous_response_id** | Option<**String**> | The unique ID of the previous response to the model. Use this to create multi-turn conversations. Learn more about [conversation state](/docs/guides/conversation-state). Cannot be used in conjunction with `conversation`. | [optional]
**tools** | Option<[**Vec<models::BetaTool>**](BetaTool.md)> | An array of tools the model may call while generating a response. You can specify which tool to use by setting the `tool_choice` parameter. | [optional]
**text** | Option<[**models::BetaResponseTextParam**](BetaResponseTextParam.md)> |  | [optional]
**reasoning** | Option<[**models::BetaReasoning**](BetaReasoning.md)> |  | [optional]
**truncation** | Option<[**models::BetaTruncationEnum**](BetaTruncationEnum.md)> |  | [optional]
**instructions** | Option<**String**> | A system (or developer) message inserted into the model's context. When used along with `previous_response_id`, the instructions from a previous response will not be carried over to the next response. This makes it simple to swap out system (or developer) messages in new responses. | [optional]
**personality** | Option<[**models::BetaPersonalityEnum**](BetaPersonalityEnum.md)> |  | [optional]
**conversation** | Option<[**models::BetaConversationParam**](BetaConversationParam.md)> |  | [optional]
**tool_choice** | Option<[**models::BetaToolChoiceParam**](BetaToolChoiceParam.md)> |  | [optional]
**parallel_tool_calls** | Option<**bool**> | Whether to allow the model to run tool calls in parallel. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


