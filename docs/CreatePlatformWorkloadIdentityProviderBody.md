# CreatePlatformWorkloadIdentityProviderBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**r#type** | **Type** | Defaults to `\"oidc\"`. (enum: oidc) | 
**issuer** | **String** |  | 
**audience** | **String** |  | 
**verification** | Option<[**models::CreatePlatformWorkloadIdentityProviderBodyVerification**](CreatePlatformWorkloadIdentityProviderBodyVerification.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**attribute_transformations** | Option<[**Vec<models::TransformationParam>**](TransformationParam.md)> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


