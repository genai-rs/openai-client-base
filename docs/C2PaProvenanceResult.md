# C2PaProvenanceResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The provenance signal type. Always `c2pa`. | 
**outcome** | [**models::ProvenanceDetectionResultApi**](ProvenanceDetectionResultApi.md) |  | 
**validation_state** | [**models::C2PaValidationStateApi**](C2PAValidationStateApi.md) |  | 
**issuer** | Option<**String**> | The C2PA manifest issuer, when available. | 
**model** | **String** | ID of the model to use | 
**generated_at** | Option<**String**> | The UTC RFC 3339 timestamp recorded by the provenance signal for when the asset was generated, when available. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


