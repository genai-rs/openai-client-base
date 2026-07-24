# BetaMultiAgentParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | **bool** | Whether to enable server-hosted multi-agent execution for this response. | 
**max_concurrent_subagents** | Option<**i32**> | `max_concurrent_subagents` sets the maximum number of subagents that can be active simultaneously across the entire agent tree. It includes all descendants—children, grandchildren, and deeper subagents—but excludes the root agent. The API does not impose a fixed upper bound on this setting. The default is `3`, which is recommended for most workloads. Multi-agent runs also have no fixed limit on tree depth or the total number of subagents created during a run. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


