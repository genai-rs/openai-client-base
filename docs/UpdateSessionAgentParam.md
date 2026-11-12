# UpdateSessionAgentParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tools** | Option<[**Vec<models::AgentToolConfigParam>**](AgentToolConfigParam.md)> | Replaces the session tools. Functions and MCP servers change on the next model call. Built-in tools change by the next turn. Omit to keep the current tools, or pass null or an empty array to clear them. | [optional]
**model** | Option<**String**> | The model for subsequent turns. Omit to keep the current model. | [optional]
**reasoning** | Option<[**models::UpdateSessionReasoningParam**](UpdateSessionReasoningParam.md)> |  | [optional]
**service_tier** | Option<[**models::ServiceTierParam**](ServiceTierParam.md)> | Omit to keep the current tier. Null resets it to auto. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


