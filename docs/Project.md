# Project

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The identifier, which can be referenced in API endpoints | 
**object** | **Object** | The object type, which is always `organization.project` (enum: organization.project) | 
**name** | Option<**String**> | The name of the project. This appears in reporting. | [optional]
**created_at** | **i32** | The Unix timestamp (in seconds) of when the project was created. | 
**archived_at** | Option<**i32**> | The Unix timestamp (in seconds) of when the project was archived or `null`. | [optional]
**status** | Option<**String**> | `active` or `archived` | [optional]
**external_key_id** | Option<**String**> | The external key associated with the project. | [optional]
**residency** | Option<[**models::PublicProjectResidency**](PublicProjectResidency.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


