# ResponseProperties

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**previous_response_id** | Option<**String**> | The unique ID of the previous response to the model. Use this to create multi-turn conversations. Learn more about [conversation state](https://platform.openai.com/docs/guides/conversation-state). Cannot be used in conjunction with `conversation`.  | [optional]
**model** | Option<**String**> | Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI offers a wide range of models with different capabilities, performance characteristics, and price points. Refer to the [model guide](https://platform.openai.com/docs/models) to browse and compare available models.  | [optional]
**reasoning** | Option<[**models::Reasoning**](Reasoning.md)> |  | [optional]
**background** | Option<**bool**> | Whether to run the model response in the background. [Learn more](https://platform.openai.com/docs/guides/background).  | [optional]
**max_output_tokens** | Option<**i32**> | An upper bound for the number of tokens that can be generated for a response, including visible output tokens and [reasoning tokens](https://platform.openai.com/docs/guides/reasoning).  | [optional]
**max_tool_calls** | Option<**i32**> | The maximum number of total calls to built-in tools that can be processed in a response. This maximum number applies across all built-in tool calls, not per individual tool. Any further attempts to call a tool by the model will be ignored.  | [optional]
**text** | Option<[**models::ResponsePropertiesText**](ResponseProperties_text.md)> |  | [optional]
**tools** | Option<[**Vec<models::Tool>**](Tool.md)> | An array of tools the model may call while generating a response. You can specify which tool to use by setting the `tool_choice` parameter.  We support the following categories of tools: - **Built-in tools**: Tools that are provided by OpenAI that extend the   model's capabilities, like [web search](https://platform.openai.com/docs/guides/tools-web-search)   or [file search](https://platform.openai.com/docs/guides/tools-file-search). Learn more about   [built-in tools](https://platform.openai.com/docs/guides/tools). - **MCP Tools**: Integrations with third-party systems via custom MCP servers   or predefined connectors such as Google Drive and SharePoint. Learn more about   [MCP Tools](https://platform.openai.com/docs/guides/tools-connectors-mcp). - **Function calls (custom tools)**: Functions that are defined by you,   enabling the model to call your own code with strongly typed arguments   and outputs. Learn more about   [function calling](https://platform.openai.com/docs/guides/function-calling). You can also use   custom tools to call your own code.  | [optional]
**tool_choice** | Option<[**models::ResponsePropertiesToolChoice**](ResponseProperties_tool_choice.md)> |  | [optional]
**prompt** | Option<[**models::Prompt**](Prompt.md)> |  | [optional]
**truncation** | Option<**String**> | The truncation strategy to use for the model response. - `auto`: If the context of this response and previous ones exceeds   the model's context window size, the model will truncate the   response to fit the context window by dropping input items in the   middle of the conversation. - `disabled` (default): If a model response will exceed the context window   size for a model, the request will fail with a 400 error.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


