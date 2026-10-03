# NetworkPolicyParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access** | [**models::NetworkAccessParam**](NetworkAccessParam.md) |  | 
**allowed_domains** | Option<**Vec<String>**> | Domains the environment may access when network access is restricted. | [optional]
**blocked_domains** | Option<**Vec<String>**> | Domains blocked for both executor and browser when access is restricted. A nonempty list requires `access: restricted` and cannot be combined with nonempty `allowed_domains`. Wildcard domains are not supported. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


