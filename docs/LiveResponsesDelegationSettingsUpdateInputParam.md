# LiveResponsesDelegationSettingsUpdateInputParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> | The Responses backend model to use for subsequent delegated requests. Omit to keep the current backend model. | [optional]
**instructions** | Option<**String**> | Instructions for the delegated Responses model, separate from Live instructions. See [backend prompting](https://developers.openai.com/api/docs/guides/live-delegation#start-with-your-existing-backend-prompt). | [optional]
**max_output_tokens** | Option<**i32**> | Maximum number of output tokens for each delegated response. | [optional]
**service_tier** | Option<[**models::LiveResponsesServiceTier**](LiveResponsesServiceTier.md)> |  | [optional]
**reasoning** | Option<[**models::LiveDelegationReasoningInputParam**](LiveDelegationReasoningInputParam.md)> |  | [optional]
**text** | Option<[**models::LiveDelegationTextInputParam**](LiveDelegationTextInputParam.md)> |  | [optional]
**tools** | Option<[**Vec<models::LiveResponsesDelegationSettingsInputParamToolsInner>**](LiveResponsesDelegationSettingsInputParam_tools_inner.md)> | Tools available to the Responses backend while it handles tasks delegated by the Live model. | [optional]
**tool_choice** | Option<[**models::LiveResponsesDelegationSettingsInputParamToolChoice**](LiveResponsesDelegationSettingsInputParam_tool_choice.md)> |  | [optional]
**parallel_tool_calls** | Option<**bool**> | Whether the delegated Responses model may request multiple tool calls in a single response. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


