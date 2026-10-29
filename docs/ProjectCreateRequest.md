# ProjectCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The friendly name of the project, this name appears in reports. | 
**geography** | Option<**String**> | Create the project with the specified data residency region. Your organization must have access to Data residency functionality in order to use. See [data residency controls](https://developers.openai.com/api/docs/guides/your-data#data-residency-controls) to review the functionality and limitations of setting this field. Deprecated: use `residency` instead. Do not provide both `geography` and `residency`. | [optional]
**residency** | Option<[**models::PublicProjectResidency**](PublicProjectResidency.md)> | Create the project with the specified residency configuration. Your organization must have access to the requested residency configuration in order to use it. See [data residency controls](https://developers.openai.com/api/docs/guides/your-data#data-residency-controls) to review the functionality and limitations of setting this field. | [optional]
**external_key_id** | Option<**String**> | External key ID to associate with the project. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


