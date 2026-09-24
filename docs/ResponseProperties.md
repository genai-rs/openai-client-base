# ResponseProperties

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**previous_response_id** | Option<**String**> | The unique ID of the previous response to the model. Use this to create multi-turn conversations. Learn more about [conversation state](https://developers.openai.com/api/docs/guides/conversation-state). Cannot be used in conjunction with `conversation`.  | [optional]
**model** | Option<**String**> | Model ID used to generate the response, like `gpt-6-astra`. OpenAI offers a wide range of models with different capabilities, performance characteristics, and price points. Refer to the [model guide](https://developers.openai.com/api/docs/models) to browse and compare available models.  | [optional]
**background** | Option<**bool**> | Whether to run the model response in the background. [Learn more](https://developers.openai.com/api/docs/guides/background).  | [optional]
**max_tool_calls** | Option<**i32**> | The maximum number of total calls to built-in tools that can be processed in a response. This maximum number applies across all built-in tool calls, not per individual tool. Any further attempts to call a tool by the model will be ignored.  | [optional]
**text** | Option<[**models::ResponseTextParam**](ResponseTextParam.md)> |  | [optional]
**tools** | Option<[**Vec<models::Tool>**](Tool.md)> | An array of tools the model may call while generating a response. You can specify which tool to use by setting the `tool_choice` parameter.  We support the following categories of tools: - **Built-in tools**: Tools that are provided by OpenAI that extend the   model's capabilities, like [web search](https://developers.openai.com/api/docs/guides/tools-web-search)   or [file search](https://developers.openai.com/api/docs/guides/tools-file-search). Learn more about   [built-in tools](https://developers.openai.com/api/docs/guides/tools). - **MCP Tools**: Integrations with third-party systems via custom MCP servers   or predefined connectors such as Google Drive and SharePoint. Learn more about   [MCP Tools](https://developers.openai.com/api/docs/guides/tools-connectors-mcp). - **Function calls (custom tools)**: Functions that are defined by you,   enabling the model to call your own code with strongly typed arguments   and outputs. Learn more about   [function calling](https://developers.openai.com/api/docs/guides/function-calling). You can also use   custom tools to call your own code.  | [optional]
**tool_choice** | Option<[**models::ToolChoiceParam**](ToolChoiceParam.md)> |  | [optional]
**prompt** | Option<[**models::Prompt**](Prompt.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


