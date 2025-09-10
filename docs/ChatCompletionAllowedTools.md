# ChatCompletionAllowedTools

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | **String** | Constrains the tools available to the model to a pre-defined set.  `auto` allows the model to pick from among the allowed tools and generate a message.  `required` requires the model to call one or more of the allowed tools.  | 
**tools** | [**Vec<std::collections::HashMap<String, serde_json::Value>>**](std::collections::HashMap.md) | A list of tool definitions that the model should be allowed to call.  For the Chat Completions API, the list of tool definitions might look like: ```json [   { \"type\": \"function\", \"function\": { \"name\": \"get_weather\" } },   { \"type\": \"function\", \"function\": { \"name\": \"get_time\" } } ] ```  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


