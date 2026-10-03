# BrowserAuthenticationHistoryRequestKindResourceBrowserAuthentication

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the object. Always `browser_authentication`. | 
**reason** | Option<**String**> | Why the agent needs the user to sign in. | 
**credential_origin** | Option<**String**> | The registered form or frame origin where values will be entered. | 
**fields** | [**Vec<models::BrowserAuthenticationFieldResource>**](BrowserAuthenticationFieldResource.md) | Controls to render. All submitted values are sensitive. | 
**options** | [**Vec<models::BrowserAuthenticationOptionResource>**](BrowserAuthenticationOptionResource.md) | Sign-in methods. Empty for a plain form. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


