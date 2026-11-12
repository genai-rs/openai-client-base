# PlatformProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | Defaults to `\"organization.workload_identity_provider\"`. (enum: organization.workload_identity_provider) | 
**id** | **String** |  | 
**r#type** | **Type** | Defaults to `\"oidc\"`. (enum: oidc) | 
**name** | **String** |  | 
**issuer** | **String** |  | 
**audience** | **String** |  | 
**verification** | [**models::PlatformProviderVerification**](PlatformProviderVerification.md) |  | 
**description** | Option<**String**> |  | 
**attribute_transformations** | [**Vec<models::Transformation>**](Transformation.md) |  | 
**enabled** | **bool** |  | 
**created_at** | **i64** |  | 
**updated_at** | **i64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


