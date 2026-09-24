# BetaResponseOutputTextAnnotationWsAdded

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> |  | [optional]
**r#type** | **String** | The type of the event. Always 'response.output_text.annotation.added'. | 
**item_id** | **String** | The unique identifier of the item to which the annotation is being added. | 
**output_index** | **i32** | The index of the output item in the response's output array. | 
**content_index** | **i32** | The index of the content part within the output item. | 
**annotation_index** | **i32** | The index of the annotation within the content part. | 
**sequence_number** | **i32** | The sequence number of this event. | 
**annotation** | [**models::BetaAnnotation**](BetaAnnotation.md) |  | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the originating `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


