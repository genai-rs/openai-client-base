# CreateFineTuningJobRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | **String** | The name of the model to fine-tune. You can select one of the [supported models](https://platform.openai.com/docs/guides/fine-tuning#which-models-can-be-fine-tuned).  | 
**training_file** | **String** | The ID of an uploaded file that contains training data.  See [upload file](https://platform.openai.com/docs/api-reference/files/create) for how to upload a file.  Your dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.  The contents of the file should differ depending on if the model uses the [chat](https://platform.openai.com/docs/api-reference/fine-tuning/chat-input), [completions](https://platform.openai.com/docs/api-reference/fine-tuning/completions-input) format, or if the fine-tuning method uses the [preference](https://platform.openai.com/docs/api-reference/fine-tuning/preference-input) format.  See the [fine-tuning guide](https://platform.openai.com/docs/guides/model-optimization) for more details.  | 
**hyperparameters** | Option<[**models::CreateFineTuningJobRequestHyperparameters**](CreateFineTuningJobRequest_hyperparameters.md)> |  | [optional]
**suffix** | Option<**String**> | A string of up to 64 characters that will be added to your fine-tuned model name.  For example, a `suffix` of \"custom-model-name\" would produce a model name like `ft:gpt-4o-mini:openai:custom-model-name:7p4lURel`.  | [optional]
**validation_file** | Option<**String**> | The ID of an uploaded file that contains validation data.  If you provide this file, the data is used to generate validation metrics periodically during fine-tuning. These metrics can be viewed in the fine-tuning results file. The same data should not be present in both train and validation files.  Your dataset must be formatted as a JSONL file. You must upload your file with the purpose `fine-tune`.  See the [fine-tuning guide](https://platform.openai.com/docs/guides/model-optimization) for more details.  | [optional]
**integrations** | Option<[**Vec<models::CreateFineTuningJobRequestIntegrationsInner>**](CreateFineTuningJobRequest_integrations_inner.md)> | A list of integrations to enable for your fine-tuning job. | [optional]
**seed** | Option<**i32**> | The seed controls the reproducibility of the job. Passing in the same seed and job parameters should produce the same results, but may differ in rare cases. If a seed is not specified, one will be generated for you.  | [optional]
**method** | Option<[**models::FineTuneMethod**](FineTuneMethod.md)> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.  Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


