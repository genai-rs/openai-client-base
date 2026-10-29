# LiveDelegationItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique ID of the delegation. Use this as delegation_id when replying to client-owned work or correlating backend events. | 
**r#type** | **Type** | The object type, always `delegation`. (enum: delegation) | 
**target** | [**models::LiveDelegationItemTarget**](LiveDelegationItemTarget.md) |  | 
**response_id** | Option<**String**> | The ID of the Responses API response associated with a Responses delegation. Omitted for client delegations. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


