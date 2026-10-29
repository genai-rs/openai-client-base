# OrganizationProjectCertificate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | The object type, which is always `organization.project.certificate`. (enum: organization.project.certificate) | 
**id** | **String** | The identifier, which can be referenced in API endpoints | 
**name** | Option<**String**> | The name of the certificate. | 
**created_at** | **i32** | The Unix timestamp (in seconds) of when the certificate was uploaded. | 
**certificate_details** | [**models::OrganizationCertificateCertificateDetails**](OrganizationCertificateCertificateDetails.md) |  | 
**active** | **bool** | Whether the certificate is currently active at the project level. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


