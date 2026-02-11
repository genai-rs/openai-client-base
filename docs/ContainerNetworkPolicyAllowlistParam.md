# ContainerNetworkPolicyAllowlistParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Allow outbound network access only to specified domains. Always `allowlist`. | 
**allowed_domains** | **Vec<String>** | A list of allowed domains when type is `allowlist`. | 
**domain_secrets** | Option<[**Vec<models::ContainerNetworkPolicyDomainSecretParam>**](ContainerNetworkPolicyDomainSecretParam.md)> | Optional domain-scoped secrets for allowlisted domains. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


