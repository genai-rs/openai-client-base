# LiveSipTrunk

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider_url** | **String** | Provider endpoint in the form `sips:host[:port][;transport=tcp]`. The default port is 5061. IPv6 addresses must be bracketed. TLS signaling is required; plaintext SIP is not supported. Do not include userinfo, a path, URI headers, or other URI parameters. Local hostnames and literal private, loopback, link-local, unspecified, multicast, and broadcast IP addresses are rejected. | 
**auth** | [**models::LiveSipTrunkAuth**](LiveSIPTrunkAuth.md) |  | 
**caller_number** | **String** | Caller phone number in E.164 format to use in the SIP From header. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


