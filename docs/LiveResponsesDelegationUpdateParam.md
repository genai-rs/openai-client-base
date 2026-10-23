# LiveResponsesDelegationUpdateParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acknowledgement_delay_ms** | Option<**i32**> | Milliseconds to wait before requesting an automatic spoken acknowledgement for a delegation. The model may still speak before this delay. Any commentary or thinking appended to the session cancels all pending acknowledgements, including those for other delegations. Defaults to 400 for Decisions sessions and 0 (request immediately) for other sessions. Omission in a session update preserves the current value. Updates apply to subsequent delegations. | [optional]
**r#type** | **String** | The delegation owner. Always `responses` for tasks handled by the Responses API. | 
**responses** | Option<[**models::LiveResponsesDelegationSettingsUpdateInputParam**](LiveResponsesDelegationSettingsUpdateInputParam.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


