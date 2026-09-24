# ResponseSteerInput

## Enum Variants

| Name | Description |
|---- | -----|
| String | Input to queue for a continuation of the response. Uses the same string or input-item shape as &#x60;response.create.input&#x60;, with a non-empty array when supplying input items.  Steering accepts only messages with the &#x60;user&#x60; role. Each message may contain only &#x60;type&#x60;, &#x60;role&#x60;, and &#x60;content&#x60;, with &#x60;content&#x60; as a string or an array of &#x60;input_text&#x60;, &#x60;input_image&#x60;, and &#x60;input_file&#x60; parts. The optional &#x60;type&#x60; must be &#x60;message&#x60;. Other roles, tool outputs, and item types are not supported for steering.  |
| Vec<models::ResponseSteerInputItem> | Input to queue for a continuation of the response. Uses the same string or input-item shape as &#x60;response.create.input&#x60;, with a non-empty array when supplying input items.  Steering accepts only messages with the &#x60;user&#x60; role. Each message may contain only &#x60;type&#x60;, &#x60;role&#x60;, and &#x60;content&#x60;, with &#x60;content&#x60; as a string or an array of &#x60;input_text&#x60;, &#x60;input_image&#x60;, and &#x60;input_file&#x60; parts. The optional &#x60;type&#x60; must be &#x60;message&#x60;. Other roles, tool outputs, and item types are not supported for steering.  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


