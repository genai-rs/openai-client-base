# Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.  Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | [optional]
**top_logprobs** | Option<**i32**> | An integer between 0 and 20 specifying the number of most likely tokens to return at each token position, each with an associated log probability.  | [optional]
**temperature** | Option<**f64**> | What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. We generally recommend altering this or `top_p` but not both.  | [optional]
**top_p** | Option<**f64**> | An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.  We generally recommend altering this or `temperature` but not both.  | [optional]
**user** | Option<**String**> | This field is being replaced by `safety_identifier` and `prompt_cache_key`. Use `prompt_cache_key` instead to maintain caching optimizations. A stable identifier for your end-users. Used to boost cache hit rates by better bucketing similar requests and  to help OpenAI detect and prevent abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#safety-identifiers).  | [optional]
**safety_identifier** | Option<**String**> | A stable identifier used to help detect users of your application that may be violating OpenAI's usage policies. The IDs should be a string that uniquely identifies each user. We recommend hashing their username or email address, in order to avoid sending us any identifying information. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#safety-identifiers).  | [optional]
**prompt_cache_key** | Option<**String**> | Used by OpenAI to cache responses for similar requests to optimize your cache hit rates. Replaces the `user` field. [Learn more](https://platform.openai.com/docs/guides/prompt-caching).  | [optional]
**service_tier** | Option<[**models::ServiceTier**](ServiceTier.md)> |  | [optional]
**prompt_cache_retention** | Option<**String**> | The retention policy for the prompt cache. Set to `24h` to enable extended prompt caching, which keeps cached prefixes active for longer, up to a maximum of 24 hours. [Learn more](https://platform.openai.com/docs/guides/prompt-caching#prompt-cache-retention).  | [optional]
**previous_response_id** | Option<**String**> | The unique ID of the previous response to the model. Use this to create multi-turn conversations. Learn more about [conversation state](https://platform.openai.com/docs/guides/conversation-state). Cannot be used in conjunction with `conversation`.  | [optional]
**model** | **String** | ID of the model to use | 
**reasoning** | Option<[**models::Reasoning**](Reasoning.md)> |  | [optional]
**background** | Option<**bool**> | Whether to run the model response in the background. [Learn more](https://platform.openai.com/docs/guides/background).  | [optional]
**max_output_tokens** | Option<**i32**> | An upper bound for the number of tokens that can be generated for a response, including visible output tokens and [reasoning tokens](https://platform.openai.com/docs/guides/reasoning).  | [optional]
**max_tool_calls** | Option<**i32**> | The maximum number of total calls to built-in tools that can be processed in a response. This maximum number applies across all built-in tool calls, not per individual tool. Any further attempts to call a tool by the model will be ignored.  | [optional]
**text** | Option<[**models::ResponseTextParam**](ResponseTextParam.md)> |  | [optional]
**tools** | Option<[**Vec<models::Tool>**](Tool.md)> | An array of tools the model may call while generating a response. You can specify which tool to use by setting the `tool_choice` parameter.  We support the following categories of tools: - **Built-in tools**: Tools that are provided by OpenAI that extend the   model's capabilities, like [web search](https://platform.openai.com/docs/guides/tools-web-search)   or [file search](https://platform.openai.com/docs/guides/tools-file-search). Learn more about   [built-in tools](https://platform.openai.com/docs/guides/tools). - **MCP Tools**: Integrations with third-party systems via custom MCP servers   or predefined connectors such as Google Drive and SharePoint. Learn more about   [MCP Tools](https://platform.openai.com/docs/guides/tools-connectors-mcp). - **Function calls (custom tools)**: Functions that are defined by you,   enabling the model to call your own code with strongly typed arguments   and outputs. Learn more about   [function calling](https://platform.openai.com/docs/guides/function-calling). You can also use   custom tools to call your own code.  | [optional]
**tool_choice** | Option<[**models::ToolChoiceParam**](ToolChoiceParam.md)> |  | [optional]
**prompt** | Option<[**models::Prompt**](Prompt.md)> |  | [optional]
**truncation** | Option<**String**> | The truncation strategy to use for the model response. - `auto`: If the input to this Response exceeds   the model's context window size, the model will truncate the   response to fit the context window by dropping items from the beginning of the conversation. - `disabled` (default): If the input size will exceed the context window   size for a model, the request will fail with a 400 error.  | [optional]
**id** | **String** | Unique identifier for this Response.  | 
**object** | **String** | The object type of this resource - always set to `response`.  | 
**status** | Option<**String**> | The status of the response generation. One of `completed`, `failed`, `in_progress`, `cancelled`, `queued`, or `incomplete`.  | [optional]
**created_at** | **f64** | Unix timestamp (in seconds) of when this Response was created.  | 
**completed_at** | Option<**f64**> | Unix timestamp (in seconds) of when this Response was completed. Only present when the status is `completed`.  | [optional]
**error** | Option<[**models::ResponseError**](ResponseError.md)> |  | 
**incomplete_details** | Option<[**models::ResponseAllOfIncompleteDetails**](Response_allOf_incomplete_details.md)> |  | 
**output** | [**Vec<serde_json::Value>**](serde_json::Value.md) | An array of content items generated by the model.  - The length and order of items in the `output` array is dependent   on the model's response. - Rather than accessing the first item in the `output` array and   assuming it's an `assistant` message with the content generated by   the model, you might consider using the `output_text` property where   supported in SDKs.  | 
**instructions** | Option<[**models::ResponseAllOfInstructions**](Response_allOf_instructions.md)> |  | 
**output_text** | Option<**String**> | SDK-only convenience property that contains the aggregated text output from all `output_text` items in the `output` array, if any are present. Supported in the Python and JavaScript SDKs.  | [optional]
**usage** | Option<[**models::ResponseUsage**](ResponseUsage.md)> |  | [optional]
**parallel_tool_calls** | **bool** | Whether to allow the model to run tool calls in parallel.  | 
**conversation** | Option<[**models::Conversation2**](Conversation2.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


