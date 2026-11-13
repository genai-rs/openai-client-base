# \SkillsApi

All URIs are relative to *https://api.openai.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_skill**](SkillsApi.md#create_skill) | **POST** /skills | Create Skill
[**create_skill_json**](SkillsApi.md#create_skill_json) | **POST** /skills | Create Skill
[**create_skill_version**](SkillsApi.md#create_skill_version) | **POST** /skills/{skill_id}/versions | Create Skill Version
[**create_skill_version_json**](SkillsApi.md#create_skill_version_json) | **POST** /skills/{skill_id}/versions | Create Skill Version
[**delete_skill**](SkillsApi.md#delete_skill) | **DELETE** /skills/{skill_id} | Delete Skill
[**delete_skill_version**](SkillsApi.md#delete_skill_version) | **DELETE** /skills/{skill_id}/versions/{version} | Delete Skill Version
[**get_skill**](SkillsApi.md#get_skill) | **GET** /skills/{skill_id} | Get Skill
[**get_skill_content**](SkillsApi.md#get_skill_content) | **GET** /skills/{skill_id}/content | Get Skill Content
[**get_skill_content_json**](SkillsApi.md#get_skill_content_json) | **GET** /skills/{skill_id}/content | Get Skill Content
[**get_skill_version**](SkillsApi.md#get_skill_version) | **GET** /skills/{skill_id}/versions/{version} | Get Skill Version
[**get_skill_version_content**](SkillsApi.md#get_skill_version_content) | **GET** /skills/{skill_id}/versions/{version}/content | Get Skill Version Content
[**get_skill_version_content_json**](SkillsApi.md#get_skill_version_content_json) | **GET** /skills/{skill_id}/versions/{version}/content | Get Skill Version Content
[**list_skill_versions**](SkillsApi.md#list_skill_versions) | **GET** /skills/{skill_id}/versions | List Skill Versions
[**list_skills**](SkillsApi.md#list_skills) | **GET** /skills | List Skills
[**update_skill_default_version**](SkillsApi.md#update_skill_default_version) | **POST** /skills/{skill_id} | Update Skill Default Version



## create_skill

> models::SkillResource create_skill(files, options)
Create Skill

Create a new skill.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**files** | [**models::CreateSkillBodyFiles**](CreateSkillBodyFiles.md) |  | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_skill().configuration(&configuration).files(..).options(&options).call().await
```

### Return type

[**models::SkillResource**](SkillResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_skill_json

> models::SkillResource create_skill_json(create_skill_body, options)
Create Skill

Create a new skill.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_skill_body** | Option<[**CreateSkillBody**](CreateSkillBody.md)> |  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_skill_json().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::SkillResource**](SkillResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_skill_version

> models::SkillVersionResource create_skill_version(skill_id, files, default, options)
Create Skill Version

Create a new immutable skill version.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill to version. | [required] |
**files** | [**models::CreateSkillVersionBodyFiles**](CreateSkillVersionBodyFiles.md) |  | [required] |
**default** | Option<**bool**> | Whether to set this version as the default. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_skill_version().configuration(&configuration).skill_id(..).files(..).options(&options).call().await
```

### Return type

[**models::SkillVersionResource**](SkillVersionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_skill_version_json

> models::SkillVersionResource create_skill_version_json(skill_id, create_skill_version_body, options)
Create Skill Version

Create a new immutable skill version.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill to version. | [required] |
**create_skill_version_body** | Option<[**CreateSkillVersionBody**](CreateSkillVersionBody.md)> |  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
create_skill_version_json().configuration(&configuration).skill_id(..).options(&options).call().await
```

### Return type

[**models::SkillVersionResource**](SkillVersionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_skill

> models::DeletedSkillResource delete_skill(skill_id, options)
Delete Skill

Delete a skill by its ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill to delete. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
delete_skill().configuration(&configuration).skill_id(..).options(&options).call().await
```

### Return type

[**models::DeletedSkillResource**](DeletedSkillResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_skill_version

> models::DeletedSkillVersionResource delete_skill_version(skill_id, version, options)
Delete Skill Version

Delete a skill version.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill. | [required] |
**version** | **String** | The skill version number. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
delete_skill_version().configuration(&configuration).skill_id(..).version(..).options(&options).call().await
```

### Return type

[**models::DeletedSkillVersionResource**](DeletedSkillVersionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skill

> models::SkillResource get_skill(skill_id, options)
Get Skill

Get a skill by its ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill to retrieve. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_skill().configuration(&configuration).skill_id(..).options(&options).call().await
```

### Return type

[**models::SkillResource**](SkillResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skill_content

> std::path::PathBuf get_skill_content(skill_id, options)
Get Skill Content

Download a skill zip bundle by its ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill to download. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_skill_content().configuration(&configuration).skill_id(..).options(&options).call().await
```

### Return type

**std::path::PathBuf**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/zip, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skill_content_json

> String get_skill_content_json(skill_id, options)
Get Skill Content

Download a skill zip bundle by its ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill to download. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_skill_content_json().configuration(&configuration).skill_id(..).options(&options).call().await
```

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skill_version

> models::SkillVersionResource get_skill_version(skill_id, version, options)
Get Skill Version

Get a specific skill version.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill. | [required] |
**version** | **String** | The version number to retrieve. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_skill_version().configuration(&configuration).skill_id(..).version(..).options(&options).call().await
```

### Return type

[**models::SkillVersionResource**](SkillVersionResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skill_version_content

> std::path::PathBuf get_skill_version_content(skill_id, version, options)
Get Skill Version Content

Download a skill version zip bundle.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill. | [required] |
**version** | **String** | The skill version number. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_skill_version_content().configuration(&configuration).skill_id(..).version(..).options(&options).call().await
```

### Return type

**std::path::PathBuf**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/zip, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skill_version_content_json

> String get_skill_version_content_json(skill_id, version, options)
Get Skill Version Content

Download a skill version zip bundle.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill. | [required] |
**version** | **String** | The skill version number. | [required] |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
get_skill_version_content_json().configuration(&configuration).skill_id(..).version(..).options(&options).call().await
```

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_skill_versions

> models::SkillVersionListResource list_skill_versions(skill_id, limit, order, after, options)
List Skill Versions

List skill versions for a skill.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill. | [required] |
**limit** | Option<**i64**> | Number of versions to retrieve. |  |
**order** | Option<[**OrderEnum**](OrderEnum.md)> | Sort order of results by version number. |  |
**after** | Option<**String**> | The skill version ID to start after. |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_skill_versions().configuration(&configuration).skill_id(..).options(&options).call().await
```

### Return type

[**models::SkillVersionListResource**](SkillVersionListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_skills

> models::SkillListResource list_skills(limit, order, after, options)
List Skills

List all skills for the current project.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of items to retrieve |  |
**order** | Option<[**OrderEnum**](OrderEnum.md)> | Sort order of results by timestamp. Use `asc` for ascending order or `desc` for descending order. |  |
**after** | Option<**String**> | Identifier for the last item from the previous pagination request |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
list_skills().configuration(&configuration).options(&options).call().await
```

### Return type

[**models::SkillListResource**](SkillListResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_skill_default_version

> models::SkillResource update_skill_default_version(skill_id, set_default_skill_version_body, options)
Update Skill Default Version

Update the default version pointer for a skill.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The identifier of the skill. | [required] |
**set_default_skill_version_body** | Option<[**SetDefaultSkillVersionBody**](SetDefaultSkillVersionBody.md)> |  |  |
**options** | Option<&timvw_rust_runtime::RequestOptions> | Per-call extra headers, query parameters and base URL, applied after everything else ([`RequestOptions`](https://docs.rs/timvw-rust-runtime/latest/timvw_rust_runtime/struct.RequestOptions.html)) |  |

```rust,ignore
update_skill_default_version().configuration(&configuration).skill_id(..).options(&options).call().await
```

### Return type

[**models::SkillResource**](SkillResource.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

