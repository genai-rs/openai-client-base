# \SkillsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_skill**](SkillsApi.md#create_skill) | **POST** /skills | Create a new skill.
[**create_skill_version**](SkillsApi.md#create_skill_version) | **POST** /skills/{skill_id}/versions | Create a new immutable skill version.
[**delete_skill**](SkillsApi.md#delete_skill) | **DELETE** /skills/{skill_id} | Delete a skill by its ID.
[**delete_skill_version**](SkillsApi.md#delete_skill_version) | **DELETE** /skills/{skill_id}/versions/{version} | Delete a skill version.
[**get_skill**](SkillsApi.md#get_skill) | **GET** /skills/{skill_id} | Get a skill by its ID.
[**get_skill_content**](SkillsApi.md#get_skill_content) | **GET** /skills/{skill_id}/content | Download a skill zip bundle by its ID.
[**get_skill_version**](SkillsApi.md#get_skill_version) | **GET** /skills/{skill_id}/versions/{version} | Get a specific skill version.
[**get_skill_version_content**](SkillsApi.md#get_skill_version_content) | **GET** /skills/{skill_id}/versions/{version}/content | Download a skill version zip bundle.
[**list_skill_versions**](SkillsApi.md#list_skill_versions) | **GET** /skills/{skill_id}/versions | List skill versions for a skill.
[**list_skills**](SkillsApi.md#list_skills) | **GET** /skills | List all skills for the current project.
[**update_skill_default_version**](SkillsApi.md#update_skill_default_version) | **POST** /skills/{skill_id} | Update the default version pointer for a skill.



## create_skill

> models::SkillResource create_skill(files)
Create a new skill.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**files** | [**models::CreateSkillBodyFiles**](CreateSkillBodyFiles.md) |  | [required] |

### Return type

[**models::SkillResource**](SkillResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_skill_version

> models::SkillVersionResource create_skill_version(skill_id, files)
Create a new immutable skill version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill to version. | [required] |
**files** | [**models::CreateSkillVersionBodyFiles**](CreateSkillVersionBodyFiles.md) |  | [required] |

### Return type

[**models::SkillVersionResource**](SkillVersionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_skill

> models::DeletedSkillResource delete_skill(skill_id)
Delete a skill by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill to delete. | [required] |

### Return type

[**models::DeletedSkillResource**](DeletedSkillResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_skill_version

> models::DeletedSkillVersionResource delete_skill_version(skill_id, version)
Delete a skill version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill. | [required] |
**version** | **String** | The skill version number. | [required] |

### Return type

[**models::DeletedSkillVersionResource**](DeletedSkillVersionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skill

> models::SkillResource get_skill(skill_id)
Get a skill by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill to retrieve. | [required] |

### Return type

[**models::SkillResource**](SkillResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skill_content

> std::path::PathBuf get_skill_content(skill_id)
Download a skill zip bundle by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill to download. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/zip, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skill_version

> models::SkillVersionResource get_skill_version(skill_id, version)
Get a specific skill version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill. | [required] |
**version** | **String** | The version number to retrieve. | [required] |

### Return type

[**models::SkillVersionResource**](SkillVersionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skill_version_content

> std::path::PathBuf get_skill_version_content(skill_id, version)
Download a skill version zip bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill. | [required] |
**version** | **String** | The skill version number. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/zip, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_skill_versions

> models::SkillVersionListResource list_skill_versions(skill_id, limit, order, after)
List skill versions for a skill.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill. | [required] |
**limit** | Option<**i32**> | Number of versions to retrieve. |  |
**order** | Option<[**OrderEnum**](.md)> | Sort order of results by version number. |  |
**after** | Option<**String**> | The skill version ID to start after. |  |

### Return type

[**models::SkillVersionListResource**](SkillVersionListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_skills

> models::SkillListResource list_skills(limit, order, after)
List all skills for the current project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of items to retrieve |  |
**order** | Option<[**OrderEnum**](.md)> | Sort order of results by timestamp. Use `asc` for ascending order or `desc` for descending order. |  |
**after** | Option<**String**> | Identifier for the last item from the previous pagination request |  |

### Return type

[**models::SkillListResource**](SkillListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_skill_default_version

> models::SkillResource update_skill_default_version(skill_id, set_default_skill_version_body)
Update the default version pointer for a skill.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill. | [required] |
**set_default_skill_version_body** | Option<[**SetDefaultSkillVersionBody**](SetDefaultSkillVersionBody.md)> |  |  |

### Return type

[**models::SkillResource**](SkillResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

