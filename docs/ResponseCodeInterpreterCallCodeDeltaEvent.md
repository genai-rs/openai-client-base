# ResponseCodeInterpreterCallCodeDeltaEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always `response.code_interpreter_call_code.delta`. | 
**output_index** | **i32** | The index of the output item in the response for which the code is being streamed. | 
**item_id** | **String** | The unique identifier of the code interpreter tool call item. | 
**delta** | **String** | The partial code snippet being streamed by the code interpreter. | 
**sequence_number** | **i32** | The sequence number of this event, used to order streaming events. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


