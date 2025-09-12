# SemanticVad

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of turn detection, `semantic_vad` to turn on Semantic VAD.  | 
**eagerness** | Option<**String**> | Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`. `low`, `medium`, and `high` have max timeouts of 8s, 4s, and 2s respectively.  | [optional]
**create_response** | Option<**bool**> | Whether or not to automatically generate a response when a VAD stop event occurs.  | [optional]
**interrupt_response** | Option<**bool**> | Whether or not to automatically interrupt any ongoing response with output to the default conversation (i.e. `conversation` of `auto`) when a VAD start event occurs.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


