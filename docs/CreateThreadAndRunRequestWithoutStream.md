# CreateThreadAndRunRequestWithoutStream

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assistant_id** | **String** | The ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) to use to execute this run. | 
**thread** | Option<[**models::CreateThreadRequest**](CreateThreadRequest.md)> |  | [optional]
**model** | Option<**String**> | The ID of the [Model](https://platform.openai.com/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used. | [optional]
**instructions** | Option<**String**> | Override the default system message of the assistant. This is useful for modifying the behavior on a per-run basis. | [optional]
**tools** | Option<[**Vec<models::AssistantTool>**](AssistantTool.md)> | Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis. | [optional]
**tool_resources** | Option<[**models::CreateThreadAndRunRequestWithoutStreamToolResources**](CreateThreadAndRunRequestWithoutStream_tool_resources.md)> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.   Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | [optional]
**temperature** | Option<**f64**> | What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.  | [optional]
**top_p** | Option<**f64**> | An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.  We generally recommend altering this or temperature but not both.  | [optional]
**max_prompt_tokens** | Option<**i32**> | The maximum number of prompt tokens that may be used over the course of the run. The run will make a best effort to use only the number of prompt tokens specified, across multiple turns of the run. If the run exceeds the number of prompt tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.  | [optional]
**max_completion_tokens** | Option<**i32**> | The maximum number of completion tokens that may be used over the course of the run. The run will make a best effort to use only the number of completion tokens specified, across multiple turns of the run. If the run exceeds the number of completion tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.  | [optional]
**truncation_strategy** | Option<[**models::CreateThreadAndRunRequestWithoutStreamTruncationStrategy**](CreateThreadAndRunRequestWithoutStream_truncation_strategy.md)> |  | [optional]
**tool_choice** | Option<[**models::CreateThreadAndRunRequestWithoutStreamToolChoice**](CreateThreadAndRunRequestWithoutStream_tool_choice.md)> |  | [optional]
**parallel_tool_calls** | Option<**bool**> | Whether to enable [parallel function calling](https://platform.openai.com/docs/guides/function-calling#configuring-parallel-function-calling) during tool use. | [optional]
**response_format** | Option<[**models::AssistantsApiResponseFormatOption**](AssistantsApiResponseFormatOption.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


