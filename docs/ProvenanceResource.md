# ProvenanceResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | [**models::ProvenanceCheckObject**](ProvenanceCheckObject.md) |  | 
**created_at** | **i32** | The Unix timestamp, in seconds, when the provenance check was created. | 
**results** | [**Vec<models::ProvenanceResourceResultsInner>**](ProvenanceResource_results_inner.md) | The provenance results that apply to the uploaded file. Image results include C2PA and SynthID; audio results include SynthID. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


