# CreateAssistantRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | **String** | ID of the model to use. You can use the [List models](https://platform.openai.com/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](https://platform.openai.com/docs/models) for descriptions of them.  | 
**name** | Option<[**models::CreateAssistantRequestName**](CreateAssistantRequest_name.md)> |  | [optional]
**description** | Option<[**models::CreateAssistantRequestDescription**](CreateAssistantRequest_description.md)> |  | [optional]
**instructions** | Option<[**models::CreateAssistantRequestInstructions**](CreateAssistantRequest_instructions.md)> |  | [optional]
**reasoning_effort** | Option<[**models::ReasoningEffort**](ReasoningEffort.md)> |  | [optional]
**tools** | Option<[**Vec<models::AssistantTool>**](AssistantTool.md)> | A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.  | [optional]
**tool_resources** | Option<[**models::CreateAssistantRequestToolResources**](CreateAssistantRequest_tool_resources.md)> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.  Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | [optional]
**temperature** | Option<[**models::CreateAssistantRequestTemperature**](CreateAssistantRequest_temperature.md)> |  | [optional]
**top_p** | Option<[**models::CreateAssistantRequestTopP**](CreateAssistantRequest_top_p.md)> |  | [optional]
**response_format** | Option<[**models::CreateAssistantRequestResponseFormat**](CreateAssistantRequest_response_format.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


