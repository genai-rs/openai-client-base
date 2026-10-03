# ComputerUseApprovalResponseParamBrowserAuthenticationSubmitParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**action** | **String** |  | 
**fields** | [**Vec<models::BrowserAuthenticationFieldValueParam>**](BrowserAuthenticationFieldValueParam.md) | Values for up to six active fields in the required action. The submitted field-value mapping and selected option must fit within 120 KiB of JSON. | 
**selected_option** | Option<**String**> | The chosen method. Required when the required action contains options. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


