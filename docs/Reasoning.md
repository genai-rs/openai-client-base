# Reasoning

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | Option<[**models::ReasoningModeEnum**](ReasoningModeEnum.md)> |  | [optional]
**effort** | Option<[**models::ReasoningEffort**](ReasoningEffort.md)> |  | [optional]
**summary** | Option<**String**> | A summary of the reasoning performed by the model. This can be useful for debugging and understanding the model's reasoning process. One of `auto`, `concise`, or `detailed`.  `concise` is supported for `computer-use-preview` models and all reasoning models after `gpt-5`.  | [optional]
**context** | Option<**String**> | Controls which reasoning items are rendered back to the model on later turns. If omitted or set to `auto`, the model determines the context mode. The `gpt-5.6` model family defaults to `all_turns`; earlier models default to `current_turn`.  When returned on a response, this is the effective reasoning context mode used for the response.  | [optional]
**generate_summary** | Option<**String**> | **Deprecated:** use `summary` instead.  A summary of the reasoning performed by the model. This can be useful for debugging and understanding the model's reasoning process. One of `auto`, `concise`, or `detailed`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


