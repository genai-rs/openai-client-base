# ToolSearchToolParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of the tool. Always `tool_search`. (enum: tool_search) | 
**execution** | Option<[**models::ToolSearchExecutionType**](ToolSearchExecutionType.md)> |  | [optional]
**description** | Option<**String**> | Description shown to the model for a client-executed tool search tool. | [optional]
**parameters** | Option<**serde_json::Value**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


