# FineTuneMethod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The type of method. Is either `supervised`, `dpo`, or `reinforcement`. (enum: supervised, dpo, reinforcement) | 
**supervised** | Option<[**models::FineTuneSupervisedMethod**](FineTuneSupervisedMethod.md)> |  | [optional]
**dpo** | Option<[**models::FineTuneDpoMethod**](FineTuneDPOMethod.md)> |  | [optional]
**reinforcement** | Option<[**models::FineTuneReinforcementMethod**](FineTuneReinforcementMethod.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


