# AdditionalToolsItemParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique ID of this additional tools item. | [optional]
**r#type** | **Type** | The item type. Always `additional_tools`. (enum: additional_tools) | 
**role** | **Role** | The role that provided the additional tools. Only `developer` is supported. (enum: developer) | 
**tools** | [**Vec<models::Tool>**](Tool.md) | A list of additional tools made available at this item. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


