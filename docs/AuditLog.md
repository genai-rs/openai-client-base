# AuditLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of this log. | 
**r#type** | [**models::AuditLogEventType**](AuditLogEventType.md) |  | 
**effective_at** | **i32** | The Unix timestamp (in seconds) of the event. | 
**project** | Option<[**models::AuditLogProject**](AuditLog_project.md)> |  | [optional]
**actor** | [**models::AuditLogActor**](AuditLogActor.md) |  | 
**api_key_created** | Option<[**models::AuditLogApiKeyCreated**](AuditLog_api_key_created.md)> |  | [optional]
**api_key_updated** | Option<[**models::AuditLogApiKeyUpdated**](AuditLog_api_key_updated.md)> |  | [optional]
**api_key_deleted** | Option<[**models::AuditLogApiKeyDeleted**](AuditLog_api_key_deleted.md)> |  | [optional]
**checkpoint_permission_created** | Option<[**models::AuditLogCheckpointPermissionCreated**](AuditLog_checkpoint_permission_created.md)> |  | [optional]
**checkpoint_permission_deleted** | Option<[**models::AuditLogCheckpointPermissionDeleted**](AuditLog_checkpoint_permission_deleted.md)> |  | [optional]
**external_key_registered** | Option<[**models::AuditLogExternalKeyRegistered**](AuditLog_external_key_registered.md)> |  | [optional]
**external_key_removed** | Option<[**models::AuditLogExternalKeyRemoved**](AuditLog_external_key_removed.md)> |  | [optional]
**group_created** | Option<[**models::AuditLogGroupCreated**](AuditLog_group_created.md)> |  | [optional]
**group_updated** | Option<[**models::AuditLogGroupUpdated**](AuditLog_group_updated.md)> |  | [optional]
**group_deleted** | Option<[**models::AuditLogGroupDeleted**](AuditLog_group_deleted.md)> |  | [optional]
**scim_enabled** | Option<[**models::AuditLogScimEnabled**](AuditLog_scim_enabled.md)> |  | [optional]
**scim_disabled** | Option<[**models::AuditLogScimDisabled**](AuditLog_scim_disabled.md)> |  | [optional]
**invite_sent** | Option<[**models::AuditLogInviteSent**](AuditLog_invite_sent.md)> |  | [optional]
**invite_accepted** | Option<[**models::AuditLogInviteAccepted**](AuditLog_invite_accepted.md)> |  | [optional]
**invite_deleted** | Option<[**models::AuditLogInviteAccepted**](AuditLog_invite_accepted.md)> |  | [optional]
**ip_allowlist_created** | Option<[**models::AuditLogIpAllowlistCreated**](AuditLog_ip_allowlist_created.md)> |  | [optional]
**ip_allowlist_updated** | Option<[**models::AuditLogIpAllowlistUpdated**](AuditLog_ip_allowlist_updated.md)> |  | [optional]
**ip_allowlist_deleted** | Option<[**models::AuditLogIpAllowlistDeleted**](AuditLog_ip_allowlist_deleted.md)> |  | [optional]
**ip_allowlist_config_activated** | Option<[**models::AuditLogIpAllowlistConfigActivated**](AuditLog_ip_allowlist_config_activated.md)> |  | [optional]
**ip_allowlist_config_deactivated** | Option<[**models::AuditLogIpAllowlistConfigDeactivated**](AuditLog_ip_allowlist_config_deactivated.md)> |  | [optional]
**login_succeeded** | Option<[**serde_json::Value**](.md)> | This event has no additional fields beyond the standard audit log attributes. | [optional]
**login_failed** | Option<[**models::AuditLogLoginFailed**](AuditLog_login_failed.md)> |  | [optional]
**logout_succeeded** | Option<[**serde_json::Value**](.md)> | This event has no additional fields beyond the standard audit log attributes. | [optional]
**logout_failed** | Option<[**models::AuditLogLoginFailed**](AuditLog_login_failed.md)> |  | [optional]
**organization_updated** | Option<[**models::AuditLogOrganizationUpdated**](AuditLog_organization_updated.md)> |  | [optional]
**project_created** | Option<[**models::AuditLogProjectCreated**](AuditLog_project_created.md)> |  | [optional]
**project_updated** | Option<[**models::AuditLogProjectUpdated**](AuditLog_project_updated.md)> |  | [optional]
**project_archived** | Option<[**models::AuditLogProjectArchived**](AuditLog_project_archived.md)> |  | [optional]
**project_deleted** | Option<[**models::AuditLogProjectArchived**](AuditLog_project_archived.md)> |  | [optional]
**rate_limit_updated** | Option<[**models::AuditLogRateLimitUpdated**](AuditLog_rate_limit_updated.md)> |  | [optional]
**rate_limit_deleted** | Option<[**models::AuditLogRateLimitDeleted**](AuditLog_rate_limit_deleted.md)> |  | [optional]
**role_created** | Option<[**models::AuditLogRoleCreated**](AuditLog_role_created.md)> |  | [optional]
**role_updated** | Option<[**models::AuditLogRoleUpdated**](AuditLog_role_updated.md)> |  | [optional]
**role_deleted** | Option<[**models::AuditLogRoleDeleted**](AuditLog_role_deleted.md)> |  | [optional]
**role_assignment_created** | Option<[**models::AuditLogRoleAssignmentCreated**](AuditLog_role_assignment_created.md)> |  | [optional]
**role_assignment_deleted** | Option<[**models::AuditLogRoleAssignmentDeleted**](AuditLog_role_assignment_deleted.md)> |  | [optional]
**service_account_created** | Option<[**models::AuditLogServiceAccountCreated**](AuditLog_service_account_created.md)> |  | [optional]
**service_account_updated** | Option<[**models::AuditLogServiceAccountUpdated**](AuditLog_service_account_updated.md)> |  | [optional]
**service_account_deleted** | Option<[**models::AuditLogServiceAccountDeleted**](AuditLog_service_account_deleted.md)> |  | [optional]
**user_added** | Option<[**models::AuditLogUserAdded**](AuditLog_user_added.md)> |  | [optional]
**user_updated** | Option<[**models::AuditLogUserUpdated**](AuditLog_user_updated.md)> |  | [optional]
**user_deleted** | Option<[**models::AuditLogUserDeleted**](AuditLog_user_deleted.md)> |  | [optional]
**certificate_created** | Option<[**models::AuditLogCertificateCreated**](AuditLog_certificate_created.md)> |  | [optional]
**certificate_updated** | Option<[**models::AuditLogCertificateCreated**](AuditLog_certificate_created.md)> |  | [optional]
**certificate_deleted** | Option<[**models::AuditLogCertificateDeleted**](AuditLog_certificate_deleted.md)> |  | [optional]
**certificates_activated** | Option<[**models::AuditLogCertificatesActivated**](AuditLog_certificates_activated.md)> |  | [optional]
**certificates_deactivated** | Option<[**models::AuditLogCertificatesActivated**](AuditLog_certificates_activated.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


