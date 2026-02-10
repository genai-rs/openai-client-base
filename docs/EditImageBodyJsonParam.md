# EditImageBodyJsonParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> | The model to use for image editing. | [optional]
**images** | [**Vec<models::ImageRefParam>**](ImageRefParam.md) | Input image references to edit. For GPT image models, you can provide up to 16 images.  | 
**mask** | Option<[**models::ImageRefParam**](ImageRefParam.md)> |  | [optional]
**prompt** | **String** | A text description of the desired image edit. | 
**n** | Option<**i32**> |  | [optional]
**quality** | Option<**String**> |  | [optional]
**input_fidelity** | Option<**String**> |  | [optional]
**size** | Option<**String**> |  | [optional]
**user** | Option<**String**> | A unique identifier representing your end-user, which can help OpenAI monitor and detect abuse.  | [optional]
**output_format** | Option<**String**> |  | [optional]
**output_compression** | Option<**i32**> |  | [optional]
**moderation** | Option<**String**> |  | [optional]
**background** | Option<**String**> |  | [optional]
**stream** | Option<**bool**> |  | [optional]
**partial_images** | Option<**i32**> | The number of partial images to generate. This parameter is used for streaming responses that return partial images. Value must be between 0 and 3. When set to 0, the response will be a single image sent in one streaming event.  Note that the final image may be sent before the full number of partial images are generated if the full image is generated more quickly.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


