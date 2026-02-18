# ChatkitWorkflow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Identifier of the workflow backing the session. | 
**version** | Option<**String**> | Specific workflow version used for the session. Defaults to null when using the latest deployment. | 
**state_variables** | Option<[**std::collections::HashMap<String, models::Object019Value>**](Object0_19_value.md)> | State variable key-value pairs applied when invoking the workflow. Defaults to null when no overrides were provided. | 
**tracing** | [**models::ChatkitWorkflowTracing**](ChatkitWorkflowTracing.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


