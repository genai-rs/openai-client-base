# TextResponseFormatConfiguration

## Enum Variants

| Name | Description |
|---- | -----|
| ResponseFormatJsonObject | An object specifying the format that the model must output.  Configuring &#x60;{ \&quot;type\&quot;: \&quot;json_schema\&quot; }&#x60; enables Structured Outputs,  which ensures the model will match your supplied JSON schema. Learn more in the  [Structured Outputs guide](/docs/guides/structured-outputs).  The default format is &#x60;{ \&quot;type\&quot;: \&quot;text\&quot; }&#x60; with no additional options.  **Not recommended for gpt-4o and newer models:**  Setting to &#x60;{ \&quot;type\&quot;: \&quot;json_object\&quot; }&#x60; enables the older JSON mode, which ensures the message the model generates is valid JSON. Using &#x60;json_schema&#x60; is preferred for models that support it.  |
| ResponseFormatText | An object specifying the format that the model must output.  Configuring &#x60;{ \&quot;type\&quot;: \&quot;json_schema\&quot; }&#x60; enables Structured Outputs,  which ensures the model will match your supplied JSON schema. Learn more in the  [Structured Outputs guide](/docs/guides/structured-outputs).  The default format is &#x60;{ \&quot;type\&quot;: \&quot;text\&quot; }&#x60; with no additional options.  **Not recommended for gpt-4o and newer models:**  Setting to &#x60;{ \&quot;type\&quot;: \&quot;json_object\&quot; }&#x60; enables the older JSON mode, which ensures the message the model generates is valid JSON. Using &#x60;json_schema&#x60; is preferred for models that support it.  |
| TextResponseFormatJsonSchema | An object specifying the format that the model must output.  Configuring &#x60;{ \&quot;type\&quot;: \&quot;json_schema\&quot; }&#x60; enables Structured Outputs,  which ensures the model will match your supplied JSON schema. Learn more in the  [Structured Outputs guide](/docs/guides/structured-outputs).  The default format is &#x60;{ \&quot;type\&quot;: \&quot;text\&quot; }&#x60; with no additional options.  **Not recommended for gpt-4o and newer models:**  Setting to &#x60;{ \&quot;type\&quot;: \&quot;json_object\&quot; }&#x60; enables the older JSON mode, which ensures the message the model generates is valid JSON. Using &#x60;json_schema&#x60; is preferred for models that support it.  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


