# CreateCompletionRequestPrompt

## Enum Variants

| Name | Description |
|---- | -----|
| String | The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.  Note that &lt;|endoftext|&gt; is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.  Defaults to &#x60;\&quot;&lt;|endoftext|&gt;\&quot;&#x60;. |
| Vec<String> | The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.  Note that &lt;|endoftext|&gt; is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.  Defaults to &#x60;\&quot;&lt;|endoftext|&gt;\&quot;&#x60;. |
| Vec<Vec<i64>> | The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.  Note that &lt;|endoftext|&gt; is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.  Defaults to &#x60;\&quot;&lt;|endoftext|&gt;\&quot;&#x60;. |
| Vec<i64> | The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.  Note that &lt;|endoftext|&gt; is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.  Defaults to &#x60;\&quot;&lt;|endoftext|&gt;\&quot;&#x60;. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


