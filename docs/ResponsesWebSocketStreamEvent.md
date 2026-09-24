# ResponsesWebSocketStreamEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The event type identifier. | 
**sequence_number** | **i32** | The sequence number of this event. | 
**delta** | **String** | The incremental input data (delta) for the custom tool call. | 
**output_index** | **i32** | The index of the output this event applies to. | 
**item_id** | **String** | Unique identifier for the API item associated with this event. | 
**code** | **String** | The final code snippet output by the code interpreter. | 
**response** | [**models::Response**](Response.md) |  | 
**content_index** | **i32** | The index of the content part within the output item. | 
**part** | [**models::ResponseReasoningSummaryPartDoneEventPart**](ResponseReasoningSummaryPartDoneEvent_part.md) |  | 
**arguments** | **String** | A JSON string containing the finalized arguments for the MCP tool call.  | 
**command_index** | **i32** | The index of the shell command that produced output. | 
**command** | **String** | The final shell command that was emitted. | 
**obfuscation** | Option<**String**> | An obfuscation string that was added to pad the event payload. | [optional]
**output** | [**Vec<models::FunctionShellCallOutputContent>**](FunctionShellCallOutputContent.md) | The output contents emitted for the shell command. | 
**item** | [**serde_json::Value**](.md) |  | 
**summary_index** | **i32** | The index of the summary part within the reasoning summary.  | 
**status** | Option<**String**> | The completion status of the summary part. Omitted when the part completed normally and set to `incomplete` when generation was interrupted.  | [optional]
**text** | **String** | The text content that is finalized.  | 
**refusal** | **String** | The refusal text that is finalized.  | 
**logprobs** | [**Vec<models::ResponseLogProb>**](ResponseLogProb.md) | The log probabilities of the tokens in the delta.  | 
**partial_image_index** | **i32** | 0-based index for the partial image (backend is 1-based, but this is 0-based for the user). | 
**partial_image_b64** | **String** | Base64-encoded partial image data, suitable for rendering as an image. | 
**size** | Option<**String**> | The image size that was used. | [optional]
**quality** | Option<**String**> | The image quality that was used. | [optional]
**background** | Option<**String**> | The background setting that was used. | [optional]
**output_format** | Option<**String**> | The output format that was used. | [optional]
**annotation_index** | **i32** | The index of the annotation within the content part. | 
**annotation** | Option<[**models::Annotation**](Annotation.md)> |  | 
**input** | **String** | The complete input data for the custom tool call. | 
**stream_id** | Option<**String**> | The WebSocket lane that emitted this event. This field is present when the originating `response.create` event supplied a `stream_id`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


