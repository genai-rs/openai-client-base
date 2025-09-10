# ResponseOutputTextAnnotationAddedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the event. Always 'response.output_text.annotation.added'. | 
**item_id** | **String** | The unique identifier of the item to which the annotation is being added. | 
**output_index** | **i32** | The index of the output item in the response's output array. | 
**content_index** | **i32** | The index of the content part within the output item. | 
**annotation_index** | **i32** | The index of the annotation within the content part. | 
**sequence_number** | **i32** | The sequence number of this event. | 
**annotation** | [**serde_json::Value**](.md) | The annotation object being added. (See annotation schema for details.) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


