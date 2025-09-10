# Certificate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | The object type.  - If creating, updating, or getting a specific certificate, the object type is `certificate`. - If listing, activating, or deactivating certificates for the organization, the object type is `organization.certificate`. - If listing, activating, or deactivating certificates for a project, the object type is `organization.project.certificate`.  | 
**id** | **String** | The identifier, which can be referenced in API endpoints | 
**name** | **String** | The name of the certificate. | 
**created_at** | **i32** | The Unix timestamp (in seconds) of when the certificate was uploaded. | 
**certificate_details** | [**models::CertificateCertificateDetails**](Certificate_certificate_details.md) |  | 
**active** | Option<**bool**> | Whether the certificate is currently active at the specified scope. Not returned when getting details for a specific certificate. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


