# TranscriptionDiarizedSegment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the segment. Always `transcript.text.segment`.  | 
**id** | **String** | Unique identifier for the segment. | 
**start** | **f32** | Start timestamp of the segment in seconds. | 
**end** | **f32** | End timestamp of the segment in seconds. | 
**text** | **String** | Transcript text for this segment. | 
**speaker** | **String** | Speaker label for this segment. When known speakers are provided, the label matches `known_speaker_names[]`. Otherwise speakers are labeled sequentially using capital letters (`A`, `B`, ...).  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


