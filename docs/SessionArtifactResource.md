# SessionArtifactResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The immutable artifact ID. | 
**object** | **String** | The object type. Always `agent.session.artifact`. | 
**session_id** | **String** | The ID of the session that owns the artifact. | 
**environment_id** | **String** | The ID of the environment that produced the artifact. | 
**turn_id** | **String** | The ID of the completed turn that published the artifact. | 
**path** | **String** | The original absolute file path in the execution environment. | 
**size_bytes** | **i64** | The immutable artifact size in bytes. | 
**created_at** | **i64** | The Unix timestamp, in seconds, when the artifact was published. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


