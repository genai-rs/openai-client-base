# McpCallItemResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The item type. Always `mcp_call`. | 
**id** | **String** | The ID of the MCP call item. | 
**turn_id** | **String** | The ID of the turn that contains this item. | 
**server_label** | **String** | The label of the MCP server. | 
**name** | **String** | The name of the MCP tool. | 
**arguments** | Option<[**serde_json::Value**](.md)> |  | 
**status** | [**models::FunctionCallStatusResource**](FunctionCallStatusResource.md) |  | 
**output** | Option<[**models::AnyOfLessThanGreaterThan**](anyOf<>.md)> | The output returned by the MCP tool, if any. | 
**error** | Option<[**models::AnyOfLessThanGreaterThan**](anyOf<>.md)> | The error returned by the MCP tool, if any. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


