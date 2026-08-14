# BetaFunctionToolParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**parameters** | Option<[**serde_json::Value**](.md)> |  | [optional]
**strict** | Option<**bool**> | Whether to enforce strict parameter validation. If omitted, Responses attempts to use strict validation when the schema is compatible, and falls back to non-strict validation otherwise. | [optional]
**r#type** | **String** |  | 
**output_schema** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | A JSON Schema describing the JSON value encoded in string outputs for this function tool. This does not describe content-array outputs. | [optional]
**defer_loading** | Option<**bool**> | Whether this function should be deferred and discovered via tool search. | [optional]
**allowed_callers** | Option<[**Vec<models::BetaCallableToolAllowedCaller>**](BetaCallableToolAllowedCaller.md)> | The tool invocation context(s). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


