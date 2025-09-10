# RealtimeBetaServerEventResponseMcpCallArgumentsDelta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The unique ID of the server event. | 
**r#type** | Option<[**serde_json::Value**](.md)> |  | 
**response_id** | **String** | The ID of the response. | 
**item_id** | **String** | The ID of the MCP tool call item. | 
**output_index** | **i32** | The index of the output item in the response. | 
**delta** | **String** | The JSON-encoded arguments delta. | 
**obfuscation** | Option<**String**> | If present, indicates the delta text was obfuscated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


