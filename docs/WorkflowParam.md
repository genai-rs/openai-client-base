# WorkflowParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Identifier for the workflow invoked by the session. | 
**version** | Option<**String**> | Specific workflow version to run. Defaults to the latest deployed version. | [optional]
**state_variables** | Option<[**std::collections::HashMap<String, models::WorkflowParamStateVariablesValue>**](WorkflowParam_state_variables_value.md)> | State variables forwarded to the workflow. Keys may be up to 64 characters, values must be primitive types, and the map defaults to an empty object. | [optional]
**tracing** | Option<[**models::WorkflowTracingParam**](WorkflowTracingParam.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


