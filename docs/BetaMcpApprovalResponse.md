# BetaMcpApprovalResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent** | Option<[**models::BetaAgentTag**](BetaAgentTag.md)> | The agent that produced this item. | [optional]
**r#type** | **Type** | The type of the item. Always `mcp_approval_response`.  (enum: mcp_approval_response) | 
**id** | Option<**String**> | The unique ID of the approval response  | [optional]
**approval_request_id** | **String** | The ID of the approval request being answered.  | 
**approve** | **bool** | Whether the request was approved.  | 
**reason** | Option<**String**> | Optional reason for the decision.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


