# AuditLogOrganizationUpdatedChangesRequested

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The organization title. | [optional]
**description** | Option<**String**> | The organization description. | [optional]
**name** | Option<**String**> | The organization name. | [optional]
**threads_ui_visibility** | Option<**String**> | Visibility of the threads page which shows messages created with the Assistants API and Playground. One of `ANY_ROLE`, `OWNERS`, or `NONE`. | [optional]
**usage_dashboard_visibility** | Option<**String**> | Visibility of the usage dashboard which shows activity and costs for your organization. One of `ANY_ROLE` or `OWNERS`. | [optional]
**api_call_logging** | Option<**String**> | How your organization logs data from supported API calls. One of `disabled`, `enabled_per_call`, `enabled_for_all_projects`, or `enabled_for_selected_projects` | [optional]
**api_call_logging_project_ids** | Option<**String**> | The list of project ids if api_call_logging is set to `enabled_for_selected_projects` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


