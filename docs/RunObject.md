# RunObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The identifier, which can be referenced in API endpoints. | 
**object** | **String** | The object type, which is always `thread.run`. | 
**created_at** | **i32** | The Unix timestamp (in seconds) for when the run was created. | 
**thread_id** | **String** | The ID of the [thread](https://platform.openai.com/docs/api-reference/threads) that was executed on as a part of this run. | 
**assistant_id** | **String** | The ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for execution of this run. | 
**status** | [**models::RunStatus**](RunStatus.md) |  | 
**required_action** | [**models::RunObjectRequiredAction**](RunObject_required_action.md) |  | 
**last_error** | [**models::RunObjectLastError**](RunObject_last_error.md) |  | 
**expires_at** | **i32** | The Unix timestamp (in seconds) for when the run will expire. | 
**started_at** | **i32** | The Unix timestamp (in seconds) for when the run was started. | 
**cancelled_at** | **i32** | The Unix timestamp (in seconds) for when the run was cancelled. | 
**failed_at** | **i32** | The Unix timestamp (in seconds) for when the run failed. | 
**completed_at** | **i32** | The Unix timestamp (in seconds) for when the run was completed. | 
**incomplete_details** | [**models::RunObjectIncompleteDetails**](RunObject_incomplete_details.md) |  | 
**model** | **String** | The model that the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for this run. | 
**instructions** | **String** | The instructions that the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for this run. | 
**tools** | [**Vec<models::AssistantTool>**](AssistantTool.md) | The list of tools that the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for this run. | 
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.  Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | 
**usage** | Option<[**models::RunCompletionUsage**](RunCompletionUsage.md)> |  | 
**temperature** | Option<**f64**> | The sampling temperature used for this run. If not set, defaults to 1. | [optional]
**top_p** | Option<**f64**> | The nucleus sampling value used for this run. If not set, defaults to 1. | [optional]
**max_prompt_tokens** | **i32** | The maximum number of prompt tokens specified to have been used over the course of the run.  | 
**max_completion_tokens** | **i32** | The maximum number of completion tokens specified to have been used over the course of the run.  | 
**truncation_strategy** | [**models::TruncationObject**](TruncationObject.md) |  | 
**tool_choice** | [**models::AssistantsApiToolChoiceOption**](AssistantsApiToolChoiceOption.md) |  | 
**parallel_tool_calls** | **bool** | Whether to enable [parallel function calling](https://platform.openai.com/docs/guides/function-calling#configuring-parallel-function-calling) during tool use. | 
**response_format** | [**models::AssistantsApiResponseFormatOption**](AssistantsApiResponseFormatOption.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


