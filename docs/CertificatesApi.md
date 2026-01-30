# \CertificatesApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_organization_certificates**](CertificatesApi.md#activate_organization_certificates) | **POST** /organization/certificates/activate | Activate certificates for organization
[**activate_project_certificates**](CertificatesApi.md#activate_project_certificates) | **POST** /organization/projects/{project_id}/certificates/activate | Activate certificates for project
[**deactivate_organization_certificates**](CertificatesApi.md#deactivate_organization_certificates) | **POST** /organization/certificates/deactivate | Deactivate certificates for organization
[**deactivate_project_certificates**](CertificatesApi.md#deactivate_project_certificates) | **POST** /organization/projects/{project_id}/certificates/deactivate | Deactivate certificates for project
[**delete_certificate**](CertificatesApi.md#delete_certificate) | **DELETE** /organization/certificates/{certificate_id} | Delete certificate
[**get_certificate**](CertificatesApi.md#get_certificate) | **GET** /organization/certificates/{certificate_id} | Get certificate
[**list_organization_certificates**](CertificatesApi.md#list_organization_certificates) | **GET** /organization/certificates | List organization certificates
[**list_project_certificates**](CertificatesApi.md#list_project_certificates) | **GET** /organization/projects/{project_id}/certificates | List project certificates
[**modify_certificate**](CertificatesApi.md#modify_certificate) | **POST** /organization/certificates/{certificate_id} | Modify certificate
[**upload_certificate**](CertificatesApi.md#upload_certificate) | **POST** /organization/certificates | Upload certificate



## activate_organization_certificates

> models::ListCertificatesResponse activate_organization_certificates(toggle_certificates_request)
Activate certificates for organization

Activate certificates at the organization level.  You can atomically and idempotently activate up to 10 certificates at a time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**toggle_certificates_request** | [**ToggleCertificatesRequest**](ToggleCertificatesRequest.md) | The certificate activation payload. | [required] |

### Return type

[**models::ListCertificatesResponse**](ListCertificatesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activate_project_certificates

> models::ListCertificatesResponse activate_project_certificates(project_id, toggle_certificates_request)
Activate certificates for project

Activate certificates at the project level.  You can atomically and idempotently activate up to 10 certificates at a time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project. | [required] |
**toggle_certificates_request** | [**ToggleCertificatesRequest**](ToggleCertificatesRequest.md) | The certificate activation payload. | [required] |

### Return type

[**models::ListCertificatesResponse**](ListCertificatesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deactivate_organization_certificates

> models::ListCertificatesResponse deactivate_organization_certificates(toggle_certificates_request)
Deactivate certificates for organization

Deactivate certificates at the organization level.  You can atomically and idempotently deactivate up to 10 certificates at a time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**toggle_certificates_request** | [**ToggleCertificatesRequest**](ToggleCertificatesRequest.md) | The certificate deactivation payload. | [required] |

### Return type

[**models::ListCertificatesResponse**](ListCertificatesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deactivate_project_certificates

> models::ListCertificatesResponse deactivate_project_certificates(project_id, toggle_certificates_request)
Deactivate certificates for project

Deactivate certificates at the project level. You can atomically and  idempotently deactivate up to 10 certificates at a time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project. | [required] |
**toggle_certificates_request** | [**ToggleCertificatesRequest**](ToggleCertificatesRequest.md) | The certificate deactivation payload. | [required] |

### Return type

[**models::ListCertificatesResponse**](ListCertificatesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_certificate

> models::DeleteCertificateResponse delete_certificate(certificate_id)
Delete certificate

Delete a certificate from the organization.  The certificate must be inactive for the organization and all projects. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **String** | Unique ID of the certificate to delete. | [required] |

### Return type

[**models::DeleteCertificateResponse**](DeleteCertificateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificate

> models::Certificate get_certificate(certificate_id, include)
Get certificate

Get a certificate that has been uploaded to the organization.  You can get a certificate regardless of whether it is active or not. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **String** | Unique ID of the certificate to retrieve. | [required] |
**include** | Option<[**Vec<String>**](String.md)> | A list of additional fields to include in the response. Currently the only supported value is `content` to fetch the PEM content of the certificate. |  |

### Return type

[**models::Certificate**](Certificate.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_certificates

> models::ListCertificatesResponse list_organization_certificates(limit, after, order)
List organization certificates

List uploaded certificates for this organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.  |  |[default to 20]
**after** | Option<**String**> | A cursor for use in pagination. `after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with obj_foo, your subsequent call can include after=obj_foo in order to fetch the next page of the list.  |  |
**order** | Option<**String**> | Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.  |  |[default to desc]

### Return type

[**models::ListCertificatesResponse**](ListCertificatesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_certificates

> models::ListCertificatesResponse list_project_certificates(project_id, limit, after, order)
List project certificates

List certificates for this project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project. | [required] |
**limit** | Option<**i32**> | A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.  |  |[default to 20]
**after** | Option<**String**> | A cursor for use in pagination. `after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with obj_foo, your subsequent call can include after=obj_foo in order to fetch the next page of the list.  |  |
**order** | Option<**String**> | Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.  |  |[default to desc]

### Return type

[**models::ListCertificatesResponse**](ListCertificatesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_certificate

> models::Certificate modify_certificate(certificate_id, modify_certificate_request)
Modify certificate

Modify a certificate. Note that only the name can be modified. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **String** | Unique ID of the certificate to modify. | [required] |
**modify_certificate_request** | [**ModifyCertificateRequest**](ModifyCertificateRequest.md) | The certificate modification payload. | [required] |

### Return type

[**models::Certificate**](Certificate.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_certificate

> models::Certificate upload_certificate(upload_certificate_request)
Upload certificate

Upload a certificate to the organization. This does **not** automatically activate the certificate.  Organizations can upload up to 50 certificates. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_certificate_request** | [**UploadCertificateRequest**](UploadCertificateRequest.md) | The certificate upload payload. | [required] |

### Return type

[**models::Certificate**](Certificate.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

