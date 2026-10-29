# ComputerUseCallItemResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | The item type. Always `computer_use_call`. (enum: computer_use_call) | 
**id** | **String** | The ID of the activity item. | 
**turn_id** | **String** | The ID of the turn that contains this item. | 
**title** | Option<**String**> | A model-generated description of the activity, when available. | 
**status** | [**models::FunctionCallStatusResource**](FunctionCallStatusResource.md) |  | 
**output** | Option<[**models::ComputerScreenshotResource**](ComputerScreenshotResource.md)> | The last screenshot emitted by the model. Null when screenshot inclusion is disabled or the call emitted no screenshot. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


