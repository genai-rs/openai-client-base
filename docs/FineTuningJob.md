# FineTuningJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The object identifier, which can be referenced in the API endpoints. | 
**created_at** | **i32** | The Unix timestamp (in seconds) for when the fine-tuning job was created. | 
**error** | Option<[**models::Object010**](Object0_10.md)> |  | 
**fine_tuned_model** | Option<**String**> | The name of the fine-tuned model that is being created. The value will be null if the fine-tuning job is still running. | 
**finished_at** | Option<**i32**> | The Unix timestamp (in seconds) for when the fine-tuning job was finished. The value will be null if the fine-tuning job is still running. | 
**hyperparameters** | [**models::FineTuningJobHyperparameters**](FineTuningJob_hyperparameters.md) |  | 
**model** | **String** | The base model that is being fine-tuned. | 
**object** | **String** | The object type, which is always \"fine_tuning.job\". | 
**organization_id** | **String** | The organization that owns the fine-tuning job. | 
**result_files** | **Vec<String>** | The compiled results file ID(s) for the fine-tuning job. You can retrieve the results with the [Files API](/docs/api-reference/files/retrieve-contents). | 
**status** | **String** | The current status of the fine-tuning job, which can be either `validating_files`, `queued`, `running`, `succeeded`, `failed`, or `cancelled`. | 
**trained_tokens** | Option<**i32**> | The total number of billable tokens processed by this fine-tuning job. The value will be null if the fine-tuning job is still running. | 
**training_file** | **String** | The file ID used for training. You can retrieve the training data with the [Files API](/docs/api-reference/files/retrieve-contents). | 
**validation_file** | Option<**String**> | The file ID used for validation. You can retrieve the validation results with the [Files API](/docs/api-reference/files/retrieve-contents). | 
**integrations** | Option<[**Vec<models::FineTuningIntegration>**](FineTuningIntegration.md)> | A list of integrations to enable for this fine-tuning job. | [optional]
**seed** | **i32** | The seed used for the fine-tuning job. | 
**estimated_finish** | Option<**i32**> | The Unix timestamp (in seconds) for when the fine-tuning job is estimated to finish. The value will be null if the fine-tuning job is not running. | [optional]
**method** | Option<[**models::FineTuneMethod**](FineTuneMethod.md)> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard.  Keys are strings with a maximum length of 64 characters. Values are strings with a maximum length of 512 characters.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


