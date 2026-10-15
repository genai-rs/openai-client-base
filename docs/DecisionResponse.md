# DecisionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | **String** | The model used. | 
**answers** | [**Vec<models::AnswerResource>**](AnswerResource.md) | Answers in question order. Any question can return a refusal instead of an answer; other questions can still receive answers. A refusal has `type: \"refusal\"` and the question's `name`, or `null` if it was unnamed. | 
**usage** | [**models::ResponseUsageResource**](ResponseUsageResource.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


