# BetaReasoning

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | Option<[**models::BetaReasoningModeEnum**](BetaReasoningModeEnum.md)> |  | [optional]
**effort** | Option<[**models::BetaReasoningEffort**](BetaReasoningEffort.md)> |  | [optional]
**summary** | Option<**Summary**> | A summary of the reasoning performed by the model. This can be useful for debugging and understanding the model's reasoning process. One of `auto`, `concise`, or `detailed`.  `concise` is supported for `computer-use-preview` models and all reasoning models after `gpt-5`.  (enum: auto, concise, detailed) | [optional]
**context** | Option<**Context**> | Controls which reasoning items are rendered back to the model on later turns. If omitted or set to `auto`, the model determines the context mode. The `gpt-5.6` model family defaults to `all_turns`; earlier models default to `current_turn`.  When returned on a response, this is the effective reasoning context mode used for the response.  (enum: auto, current_turn, all_turns) | [optional]
**generate_summary** | Option<**GenerateSummary**> | **Deprecated:** use `summary` instead.  A summary of the reasoning performed by the model. This can be useful for debugging and understanding the model's reasoning process. One of `auto`, `concise`, or `detailed`.  (enum: auto, concise, detailed) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


