# BetaAdditionalToolsItemParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTagParam**](Beta_AgentTagParam.md)> |  | [optional]
**id** | Option<**String**> | The unique ID of this additional tools item. | [optional]
**r#type** | **String** | The item type. Always `additional_tools`. | 
**role** | **String** | The role that provided the additional tools. Only `developer` is supported. | 
**tools** | [**Vec<models::BetaTool>**](BetaTool.md) | A list of additional tools made available at this item. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


