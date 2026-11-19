#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod apply_patch_tool_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::apply_patch_tool_param::ApplyPatchToolParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod approximate_location;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::approximate_location::ApproximateLocation;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod auto_code_interpreter_tool_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::auto_code_interpreter_tool_param::AutoCodeInterpreterToolParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod auto_code_interpreter_tool_param_network_policy;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::auto_code_interpreter_tool_param_network_policy::AutoCodeInterpreterToolParamNetworkPolicy;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod callable_tool_allowed_caller;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::callable_tool_allowed_caller::CallableToolAllowedCaller;
#[cfg(any(feature = "chat-completion-types", feature = "evals-types"))]
pub mod chat_completion_tool;
#[cfg(any(feature = "chat-completion-types", feature = "evals-types"))]
pub use self::chat_completion_tool::ChatCompletionTool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod code_interpreter_tool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::code_interpreter_tool::CodeInterpreterTool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod code_interpreter_tool_container;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::code_interpreter_tool_container::CodeInterpreterToolContainer;
#[cfg(any(
    feature = "evals-types",
    feature = "responses-types",
    feature = "vectorstore-types"
))]
pub mod comparison_filter;
#[cfg(any(
    feature = "evals-types",
    feature = "responses-types",
    feature = "vectorstore-types"
))]
pub use self::comparison_filter::ComparisonFilter;
#[cfg(any(
    feature = "evals-types",
    feature = "responses-types",
    feature = "vectorstore-types"
))]
pub mod comparison_filter_value;
#[cfg(any(
    feature = "evals-types",
    feature = "responses-types",
    feature = "vectorstore-types"
))]
pub use self::comparison_filter_value::ComparisonFilterValue;
#[cfg(any(
    feature = "evals-types",
    feature = "responses-types",
    feature = "vectorstore-types"
))]
pub mod comparison_filter_value_one_of_inner;
#[cfg(any(
    feature = "evals-types",
    feature = "responses-types",
    feature = "vectorstore-types"
))]
pub use self::comparison_filter_value_one_of_inner::ComparisonFilterValueOneOfInner;
#[cfg(any(
    feature = "evals-types",
    feature = "responses-types",
    feature = "vectorstore-types"
))]
pub mod compound_filter;
#[cfg(any(
    feature = "evals-types",
    feature = "responses-types",
    feature = "vectorstore-types"
))]
pub use self::compound_filter::CompoundFilter;
#[cfg(any(
    feature = "evals-types",
    feature = "responses-types",
    feature = "vectorstore-types"
))]
pub mod compound_filter_filters_inner;
#[cfg(any(
    feature = "evals-types",
    feature = "responses-types",
    feature = "vectorstore-types"
))]
pub use self::compound_filter_filters_inner::CompoundFilterFiltersInner;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod computer_environment;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::computer_environment::ComputerEnvironment;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod computer_tool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::computer_tool::ComputerTool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod computer_use_preview_tool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::computer_use_preview_tool::ComputerUsePreviewTool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod container_auto_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::container_auto_param::ContainerAutoParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod container_auto_param_network_policy;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::container_auto_param_network_policy::ContainerAutoParamNetworkPolicy;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod container_auto_param_skills_inner;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::container_auto_param_skills_inner::ContainerAutoParamSkillsInner;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod container_memory_limit;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::container_memory_limit::ContainerMemoryLimit;
#[cfg(any(
    feature = "container-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub mod container_network_policy_allowlist_param;
#[cfg(any(
    feature = "container-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub use self::container_network_policy_allowlist_param::ContainerNetworkPolicyAllowlistParam;
#[cfg(any(
    feature = "container-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub mod container_network_policy_disabled_param;
#[cfg(any(
    feature = "container-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub use self::container_network_policy_disabled_param::ContainerNetworkPolicyDisabledParam;
#[cfg(any(
    feature = "container-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub mod container_network_policy_domain_secret_param;
#[cfg(any(
    feature = "container-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub use self::container_network_policy_domain_secret_param::ContainerNetworkPolicyDomainSecretParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod container_reference_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::container_reference_param::ContainerReferenceParam;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod create_model_response_properties;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::create_model_response_properties::CreateModelResponseProperties;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod custom_grammar_format_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::custom_grammar_format_param::CustomGrammarFormatParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod custom_text_format_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::custom_text_format_param::CustomTextFormatParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod custom_tool_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::custom_tool_param::CustomToolParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod custom_tool_param_format;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::custom_tool_param_format::CustomToolParamFormat;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod easy_input_message;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::easy_input_message::EasyInputMessage;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod easy_input_message_content;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::easy_input_message_content::EasyInputMessageContent;
#[cfg(any(
    feature = "administration-types",
    feature = "assistants-types",
    feature = "audio-types",
    feature = "batch-types",
    feature = "chat-completion-types",
    feature = "chatkit-types",
    feature = "completions-types",
    feature = "container-types",
    feature = "content-provenance-checks-types",
    feature = "embedding-types",
    feature = "evals-types",
    feature = "file-types",
    feature = "finetuning-types",
    feature = "image-types",
    feature = "live-types",
    feature = "model-types",
    feature = "moderation-types",
    feature = "realtime-types",
    feature = "responses-types",
    feature = "safety-types",
    feature = "skill-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "video-types",
    feature = "webhook-types"
))]
pub mod error;
#[cfg(any(
    feature = "administration-types",
    feature = "assistants-types",
    feature = "audio-types",
    feature = "batch-types",
    feature = "chat-completion-types",
    feature = "chatkit-types",
    feature = "completions-types",
    feature = "container-types",
    feature = "content-provenance-checks-types",
    feature = "embedding-types",
    feature = "evals-types",
    feature = "file-types",
    feature = "finetuning-types",
    feature = "image-types",
    feature = "live-types",
    feature = "model-types",
    feature = "moderation-types",
    feature = "realtime-types",
    feature = "responses-types",
    feature = "safety-types",
    feature = "skill-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "video-types",
    feature = "webhook-types"
))]
pub use self::error::Error;
#[cfg(any(feature = "agents-types", feature = "vaults-types"))]
pub mod error_body_resource;
#[cfg(any(feature = "agents-types", feature = "vaults-types"))]
pub use self::error_body_resource::ErrorBodyResource;
#[cfg(any(
    feature = "administration-types",
    feature = "assistants-types",
    feature = "audio-types",
    feature = "batch-types",
    feature = "chat-completion-types",
    feature = "chatkit-types",
    feature = "completions-types",
    feature = "container-types",
    feature = "content-provenance-checks-types",
    feature = "embedding-types",
    feature = "evals-types",
    feature = "file-types",
    feature = "finetuning-types",
    feature = "image-types",
    feature = "live-types",
    feature = "model-types",
    feature = "moderation-types",
    feature = "realtime-types",
    feature = "responses-types",
    feature = "safety-types",
    feature = "skill-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "video-types",
    feature = "webhook-types"
))]
pub mod error_response;
#[cfg(any(
    feature = "administration-types",
    feature = "assistants-types",
    feature = "audio-types",
    feature = "batch-types",
    feature = "chat-completion-types",
    feature = "chatkit-types",
    feature = "completions-types",
    feature = "container-types",
    feature = "content-provenance-checks-types",
    feature = "embedding-types",
    feature = "evals-types",
    feature = "file-types",
    feature = "finetuning-types",
    feature = "image-types",
    feature = "live-types",
    feature = "model-types",
    feature = "moderation-types",
    feature = "realtime-types",
    feature = "responses-types",
    feature = "safety-types",
    feature = "skill-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "video-types",
    feature = "webhook-types"
))]
pub use self::error_response::ErrorResponse;
#[cfg(any(feature = "agents-types", feature = "vaults-types"))]
pub mod error_response2;
#[cfg(any(feature = "agents-types", feature = "vaults-types"))]
pub use self::error_response2::ErrorResponse2;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod file_input_detail;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::file_input_detail::FileInputDetail;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod file_search_tool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::file_search_tool::FileSearchTool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod filters;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::filters::Filters;
#[cfg(any(
    feature = "assistants-types",
    feature = "chat-completion-types",
    feature = "evals-types"
))]
pub mod function_object;
#[cfg(any(
    feature = "assistants-types",
    feature = "chat-completion-types",
    feature = "evals-types"
))]
pub use self::function_object::FunctionObject;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod function_shell_tool_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::function_shell_tool_param::FunctionShellToolParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod function_shell_tool_param_environment;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::function_shell_tool_param_environment::FunctionShellToolParamEnvironment;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod function_tool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::function_tool::FunctionTool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod function_tool_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::function_tool_param::FunctionToolParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod grammar_syntax1;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::grammar_syntax1::GrammarSyntax1;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod hybrid_search_options;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::hybrid_search_options::HybridSearchOptions;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod image_detail;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::image_detail::ImageDetail;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod image_gen_action_enum;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::image_gen_action_enum::ImageGenActionEnum;
#[cfg(any(feature = "image-types", feature = "responses-types"))]
pub mod image_gen_input_usage_details;
#[cfg(any(feature = "image-types", feature = "responses-types"))]
pub use self::image_gen_input_usage_details::ImageGenInputUsageDetails;
#[cfg(any(feature = "image-types", feature = "responses-types"))]
pub mod image_gen_output_tokens_details;
#[cfg(any(feature = "image-types", feature = "responses-types"))]
pub use self::image_gen_output_tokens_details::ImageGenOutputTokensDetails;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod image_gen_tool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::image_gen_tool::ImageGenTool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod image_gen_tool_input_image_mask;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::image_gen_tool_input_image_mask::ImageGenToolInputImageMask;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod image_gen_tool_size;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::image_gen_tool_size::ImageGenToolSize;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod image_gen_tool_size_text_variant_enum;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::image_gen_tool_size_text_variant_enum::ImageGenToolSizeTextVariantEnum;
#[cfg(any(
    feature = "container-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub mod inline_skill_param;
#[cfg(any(
    feature = "container-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub use self::inline_skill_param::InlineSkillParam;
#[cfg(any(
    feature = "container-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub mod inline_skill_source_param;
#[cfg(any(
    feature = "container-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub use self::inline_skill_source_param::InlineSkillSourceParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod input_content;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::input_content::InputContent;
#[cfg(any(
    feature = "evals-types",
    feature = "image-types",
    feature = "responses-types"
))]
pub mod input_fidelity;
#[cfg(any(
    feature = "evals-types",
    feature = "image-types",
    feature = "responses-types"
))]
pub use self::input_fidelity::InputFidelity;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod input_file_content;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::input_file_content::InputFileContent;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod input_image_content;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::input_image_content::InputImageContent;
#[cfg(any(
    feature = "evals-types",
    feature = "finetuning-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod input_text_content;
#[cfg(any(
    feature = "evals-types",
    feature = "finetuning-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::input_text_content::InputTextContent;
#[cfg(any(
    feature = "assistants-types",
    feature = "chat-completion-types",
    feature = "evals-types"
))]
pub mod json_schema;
#[cfg(any(
    feature = "assistants-types",
    feature = "chat-completion-types",
    feature = "evals-types"
))]
pub use self::json_schema::JsonSchema;
#[cfg(any(feature = "agents-types", feature = "vaults-types"))]
pub mod list_order_param;
#[cfg(any(feature = "agents-types", feature = "vaults-types"))]
pub use self::list_order_param::ListOrderParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod local_environment_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::local_environment_param::LocalEnvironmentParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod local_shell_tool_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::local_shell_tool_param::LocalShellToolParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod local_skill_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::local_skill_param::LocalSkillParam;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod mcp_tool;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::mcp_tool::McpTool;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod mcp_tool_allowed_tools;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::mcp_tool_allowed_tools::McpToolAllowedTools;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod mcp_tool_approval_filter;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::mcp_tool_approval_filter::McpToolApprovalFilter;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod mcp_tool_filter;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::mcp_tool_filter::McpToolFilter;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod mcp_tool_require_approval;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::mcp_tool_require_approval::McpToolRequireApproval;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod mcp_tool_require_approval_mcp_tool_approval_setting_enum;
#[cfg(any(
    feature = "evals-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::mcp_tool_require_approval_mcp_tool_approval_setting_enum::McpToolRequireApprovalMcpToolApprovalSettingEnum;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod message_phase;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::message_phase::MessagePhase;
#[cfg(any(
    feature = "administration-types",
    feature = "assistants-types",
    feature = "audio-types",
    feature = "batch-types",
    feature = "chat-completion-types",
    feature = "chatkit-types",
    feature = "completions-types",
    feature = "container-types",
    feature = "content-provenance-checks-types",
    feature = "embedding-types",
    feature = "evals-types",
    feature = "file-types",
    feature = "finetuning-types",
    feature = "image-types",
    feature = "live-types",
    feature = "model-types",
    feature = "moderation-types",
    feature = "realtime-types",
    feature = "responses-types",
    feature = "safety-types",
    feature = "skill-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "video-types",
    feature = "webhook-types"
))]
pub mod misalignment_error_details_resource;
#[cfg(any(
    feature = "administration-types",
    feature = "assistants-types",
    feature = "audio-types",
    feature = "batch-types",
    feature = "chat-completion-types",
    feature = "chatkit-types",
    feature = "completions-types",
    feature = "container-types",
    feature = "content-provenance-checks-types",
    feature = "embedding-types",
    feature = "evals-types",
    feature = "file-types",
    feature = "finetuning-types",
    feature = "image-types",
    feature = "live-types",
    feature = "model-types",
    feature = "moderation-types",
    feature = "realtime-types",
    feature = "responses-types",
    feature = "safety-types",
    feature = "skill-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "video-types",
    feature = "webhook-types"
))]
pub use self::misalignment_error_details_resource::MisalignmentErrorDetailsResource;
#[cfg(any(
    feature = "administration-types",
    feature = "assistants-types",
    feature = "audio-types",
    feature = "batch-types",
    feature = "chat-completion-types",
    feature = "chatkit-types",
    feature = "completions-types",
    feature = "container-types",
    feature = "content-provenance-checks-types",
    feature = "embedding-types",
    feature = "evals-types",
    feature = "file-types",
    feature = "finetuning-types",
    feature = "image-types",
    feature = "live-types",
    feature = "model-types",
    feature = "moderation-types",
    feature = "realtime-types",
    feature = "responses-types",
    feature = "safety-types",
    feature = "skill-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "video-types",
    feature = "webhook-types"
))]
pub mod _misalignment_error_type;
#[cfg(any(
    feature = "administration-types",
    feature = "assistants-types",
    feature = "audio-types",
    feature = "batch-types",
    feature = "chat-completion-types",
    feature = "chatkit-types",
    feature = "completions-types",
    feature = "container-types",
    feature = "content-provenance-checks-types",
    feature = "embedding-types",
    feature = "evals-types",
    feature = "file-types",
    feature = "finetuning-types",
    feature = "image-types",
    feature = "live-types",
    feature = "model-types",
    feature = "moderation-types",
    feature = "realtime-types",
    feature = "responses-types",
    feature = "safety-types",
    feature = "skill-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "video-types",
    feature = "webhook-types"
))]
pub use self::_misalignment_error_type::MisalignmentErrorType;
#[cfg(any(
    feature = "administration-types",
    feature = "assistants-types",
    feature = "audio-types",
    feature = "batch-types",
    feature = "chat-completion-types",
    feature = "chatkit-types",
    feature = "completions-types",
    feature = "container-types",
    feature = "content-provenance-checks-types",
    feature = "embedding-types",
    feature = "evals-types",
    feature = "file-types",
    feature = "finetuning-types",
    feature = "image-types",
    feature = "live-types",
    feature = "model-types",
    feature = "moderation-types",
    feature = "realtime-types",
    feature = "responses-types",
    feature = "safety-types",
    feature = "skill-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "video-types",
    feature = "webhook-types"
))]
pub mod misalignment_error_type_text_variant_enum;
#[cfg(any(
    feature = "administration-types",
    feature = "assistants-types",
    feature = "audio-types",
    feature = "batch-types",
    feature = "chat-completion-types",
    feature = "chatkit-types",
    feature = "completions-types",
    feature = "container-types",
    feature = "content-provenance-checks-types",
    feature = "embedding-types",
    feature = "evals-types",
    feature = "file-types",
    feature = "finetuning-types",
    feature = "image-types",
    feature = "live-types",
    feature = "model-types",
    feature = "moderation-types",
    feature = "realtime-types",
    feature = "responses-types",
    feature = "safety-types",
    feature = "skill-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "video-types",
    feature = "webhook-types"
))]
pub use self::misalignment_error_type_text_variant_enum::MisalignmentErrorTypeTextVariantEnum;
#[cfg(any(
    feature = "administration-types",
    feature = "assistants-types",
    feature = "audio-types",
    feature = "batch-types",
    feature = "chat-completion-types",
    feature = "chatkit-types",
    feature = "completions-types",
    feature = "container-types",
    feature = "content-provenance-checks-types",
    feature = "embedding-types",
    feature = "evals-types",
    feature = "file-types",
    feature = "finetuning-types",
    feature = "image-types",
    feature = "live-types",
    feature = "model-types",
    feature = "moderation-types",
    feature = "realtime-types",
    feature = "responses-types",
    feature = "safety-types",
    feature = "skill-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "video-types",
    feature = "webhook-types"
))]
pub mod _misalignment_steer;
#[cfg(any(
    feature = "administration-types",
    feature = "assistants-types",
    feature = "audio-types",
    feature = "batch-types",
    feature = "chat-completion-types",
    feature = "chatkit-types",
    feature = "completions-types",
    feature = "container-types",
    feature = "content-provenance-checks-types",
    feature = "embedding-types",
    feature = "evals-types",
    feature = "file-types",
    feature = "finetuning-types",
    feature = "image-types",
    feature = "live-types",
    feature = "model-types",
    feature = "moderation-types",
    feature = "realtime-types",
    feature = "responses-types",
    feature = "safety-types",
    feature = "skill-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "video-types",
    feature = "webhook-types"
))]
pub use self::_misalignment_steer::MisalignmentSteer;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod model_response_properties;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::model_response_properties::ModelResponseProperties;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod moderation_config_param;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::moderation_config_param::ModerationConfigParam;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod moderation_input_type;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::moderation_input_type::ModerationInputType;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod moderation_mode;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::moderation_mode::ModerationMode;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod moderation_param;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::moderation_param::ModerationParam;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod moderation_policy_param;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::moderation_policy_param::ModerationPolicyParam;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod moderation_result_body;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::moderation_result_body::ModerationResultBody;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod namespace_tool_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::namespace_tool_param::NamespaceToolParam;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod namespace_tool_param_tools_inner;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::namespace_tool_param_tools_inner::NamespaceToolParamToolsInner;
#[cfg(any(
    feature = "administration-types",
    feature = "chatkit-types",
    feature = "skill-types",
    feature = "video-types"
))]
pub mod order_enum;
#[cfg(any(
    feature = "administration-types",
    feature = "chatkit-types",
    feature = "skill-types",
    feature = "video-types"
))]
pub use self::order_enum::OrderEnum;
#[cfg(any(feature = "administration-types", feature = "model-types"))]
pub mod permission_error_response;
#[cfg(any(feature = "administration-types", feature = "model-types"))]
pub use self::permission_error_response::PermissionErrorResponse;
#[cfg(any(feature = "administration-types", feature = "model-types"))]
pub mod permission_error_response_error;
#[cfg(any(feature = "administration-types", feature = "model-types"))]
pub use self::permission_error_response_error::PermissionErrorResponseError;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod programmatic_tool_calling_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::programmatic_tool_calling_param::ProgrammaticToolCallingParam;
#[cfg(any(feature = "realtime-types", feature = "responses-types"))]
pub mod prompt;
#[cfg(any(feature = "realtime-types", feature = "responses-types"))]
pub use self::prompt::Prompt;
#[cfg(any(
    feature = "evals-types",
    feature = "finetuning-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub mod prompt_cache_breakpoint_config;
#[cfg(any(
    feature = "evals-types",
    feature = "finetuning-types",
    feature = "realtime-types",
    feature = "responses-types"
))]
pub use self::prompt_cache_breakpoint_config::PromptCacheBreakpointConfig;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod prompt_cache_breakpoint_param;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::prompt_cache_breakpoint_param::PromptCacheBreakpointParam;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod prompt_cache_mode_enum;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::prompt_cache_mode_enum::PromptCacheModeEnum;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod prompt_cache_options_param;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::prompt_cache_options_param::PromptCacheOptionsParam;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod prompt_cache_ttl_enum;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::prompt_cache_ttl_enum::PromptCacheTtlEnum;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod ranker_version_type;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::ranker_version_type::RankerVersionType;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod ranking_options;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::ranking_options::RankingOptions;
#[cfg(any(
    feature = "assistants-types",
    feature = "chat-completion-types",
    feature = "evals-types",
    feature = "finetuning-types",
    feature = "responses-types"
))]
pub mod reasoning_effort;
#[cfg(any(
    feature = "assistants-types",
    feature = "chat-completion-types",
    feature = "evals-types",
    feature = "finetuning-types",
    feature = "responses-types"
))]
pub use self::reasoning_effort::ReasoningEffort;
#[cfg(any(
    feature = "evals-types",
    feature = "image-types",
    feature = "responses-types"
))]
pub mod reasoning_effort_enum;
#[cfg(any(
    feature = "evals-types",
    feature = "image-types",
    feature = "responses-types"
))]
pub use self::reasoning_effort_enum::ReasoningEffortEnum;
#[cfg(any(
    feature = "assistants-types",
    feature = "chat-completion-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub mod response_format_json_object;
#[cfg(any(
    feature = "assistants-types",
    feature = "chat-completion-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub use self::response_format_json_object::ResponseFormatJsonObject;
#[cfg(any(
    feature = "assistants-types",
    feature = "chat-completion-types",
    feature = "evals-types"
))]
pub mod response_format_json_schema;
#[cfg(any(
    feature = "assistants-types",
    feature = "chat-completion-types",
    feature = "evals-types"
))]
pub use self::response_format_json_schema::ResponseFormatJsonSchema;
#[cfg(any(
    feature = "assistants-types",
    feature = "chat-completion-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub mod response_format_text;
#[cfg(any(
    feature = "assistants-types",
    feature = "chat-completion-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub use self::response_format_text::ResponseFormatText;
#[cfg(any(feature = "realtime-types", feature = "responses-types"))]
pub mod response_prompt_variables_value;
#[cfg(any(feature = "realtime-types", feature = "responses-types"))]
pub use self::response_prompt_variables_value::ResponsePromptVariablesValue;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod search_content_type;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::search_content_type::SearchContentType;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod search_context_size;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::search_context_size::SearchContextSize;
#[cfg(any(
    feature = "container-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub mod skill_reference_param;
#[cfg(any(
    feature = "container-types",
    feature = "evals-types",
    feature = "responses-types"
))]
pub use self::skill_reference_param::SkillReferenceParam;
#[cfg(any(
    feature = "evals-types",
    feature = "image-types",
    feature = "responses-types"
))]
pub mod svg_param;
#[cfg(any(
    feature = "evals-types",
    feature = "image-types",
    feature = "responses-types"
))]
pub use self::svg_param::SvgParam;
#[cfg(any(
    feature = "evals-types",
    feature = "image-types",
    feature = "responses-types"
))]
pub mod svg_reasoning_param;
#[cfg(any(
    feature = "evals-types",
    feature = "image-types",
    feature = "responses-types"
))]
pub use self::svg_reasoning_param::SvgReasoningParam;
#[cfg(any(feature = "image-types", feature = "responses-types"))]
pub mod svg_usage;
#[cfg(any(feature = "image-types", feature = "responses-types"))]
pub use self::svg_usage::SvgUsage;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod text_response_format_configuration;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::text_response_format_configuration::TextResponseFormatConfiguration;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod text_response_format_json_schema;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::text_response_format_json_schema::TextResponseFormatJsonSchema;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod tool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::tool::Tool;
#[cfg(any(feature = "realtime-types", feature = "responses-types"))]
pub mod tool_choice_function;
#[cfg(any(feature = "realtime-types", feature = "responses-types"))]
pub use self::tool_choice_function::ToolChoiceFunction;
#[cfg(any(feature = "realtime-types", feature = "responses-types"))]
pub mod tool_choice_mcp;
#[cfg(any(feature = "realtime-types", feature = "responses-types"))]
pub use self::tool_choice_mcp::ToolChoiceMcp;
#[cfg(any(feature = "realtime-types", feature = "responses-types"))]
pub mod tool_choice_options;
#[cfg(any(feature = "realtime-types", feature = "responses-types"))]
pub use self::tool_choice_options::ToolChoiceOptions;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod tool_search_execution_type;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::tool_search_execution_type::ToolSearchExecutionType;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod tool_search_tool_param;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::tool_search_tool_param::ToolSearchToolParam;
#[cfg(any(feature = "responses-types", feature = "vectorstore-types"))]
pub mod vector_store_file_attributes_value;
#[cfg(any(feature = "responses-types", feature = "vectorstore-types"))]
pub use self::vector_store_file_attributes_value::VectorStoreFileAttributesValue;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub mod verbosity;
#[cfg(any(feature = "chat-completion-types", feature = "responses-types"))]
pub use self::verbosity::Verbosity;
#[cfg(any(feature = "chat-completion-types", feature = "realtime-types"))]
pub mod voice_ids_or_custom_voice;
#[cfg(any(feature = "chat-completion-types", feature = "realtime-types"))]
pub use self::voice_ids_or_custom_voice::VoiceIdsOrCustomVoice;
#[cfg(any(feature = "chat-completion-types", feature = "realtime-types"))]
pub mod voice_ids_or_custom_voice_object1;
#[cfg(any(feature = "chat-completion-types", feature = "realtime-types"))]
pub use self::voice_ids_or_custom_voice_object1::VoiceIdsOrCustomVoiceObject1;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod web_search_approximate_location;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::web_search_approximate_location::WebSearchApproximateLocation;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod web_search_preview_tool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::web_search_preview_tool::WebSearchPreviewTool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod web_search_tool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::web_search_tool::WebSearchTool;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub mod web_search_tool_filters;
#[cfg(any(feature = "evals-types", feature = "responses-types"))]
pub use self::web_search_tool_filters::WebSearchToolFilters;
pub mod validation;

/// Serde for a Unix timestamp: a [`jiff::Timestamp`] in Rust, a number of seconds since the epoch in JSON.
///
/// The generated models name these modules in `#[serde(with = "...")]`, and your own types can too:
/// `unix_time` for a `Timestamp`, [`unix_time::option`](option) for an `Option<Timestamp>` and
/// [`unix_time::double_option`](double_option) for an `Option<Option<Timestamp>>` (absent, `null` or a value).
/// They write whole seconds, as an integer. [`fractional`] has the same three for a JSON `number`, which keeps the
/// fraction of a second. All of them read an integer or a fraction, and fail on a value that is not a time in
/// seconds: milliseconds are far outside the years a `Timestamp` holds.
pub mod unix_time {
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    fn from_seconds<E: de::Error>(seconds: i64, nanoseconds: i32) -> Result<jiff::Timestamp, E> {
        jiff::Timestamp::new(seconds, nanoseconds).map_err(|error| {
            E::custom(format!(
                "{seconds} is not a Unix timestamp in seconds: {error}"
            ))
        })
    }

    struct Seconds;

    impl de::Visitor<'_> for Seconds {
        type Value = jiff::Timestamp;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a Unix timestamp, in seconds")
        }

        fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
            from_seconds(value, 0)
        }

        fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
            let seconds = i64::try_from(value)
                .map_err(|_| E::custom(format!("{value} is not a Unix timestamp in seconds")))?;
            from_seconds(seconds, 0)
        }

        fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
            let seconds = value.floor();
            if !value.is_finite() || seconds < i64::MIN as f64 || seconds > i64::MAX as f64 {
                return Err(E::custom(format!(
                    "{value} is not a Unix timestamp in seconds"
                )));
            }
            let nanoseconds = ((value - seconds) * 1e9).round().min(999_999_999.0);
            from_seconds(seconds as i64, nanoseconds as i32)
        }
    }

    /// Whole seconds, as an integer.
    struct Whole(jiff::Timestamp);

    impl Serialize for Whole {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_i64(self.0.as_second())
        }
    }

    impl<'de> Deserialize<'de> for Whole {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_i64(Seconds).map(Whole)
        }
    }

    /// Seconds with their fraction: an integer when there is none.
    struct Fraction(jiff::Timestamp);

    impl Serialize for Fraction {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self.0.subsec_nanosecond() {
                0 => serializer.serialize_i64(self.0.as_second()),
                nanoseconds => serializer
                    .serialize_f64(self.0.as_second() as f64 + f64::from(nanoseconds) / 1e9),
            }
        }
    }

    impl<'de> Deserialize<'de> for Fraction {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_f64(Seconds).map(Fraction)
        }
    }

    macro_rules! with {
        ($wire:ident) => {
            pub fn serialize<S: Serializer>(
                value: &jiff::Timestamp,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                $wire(*value).serialize(serializer)
            }

            pub fn deserialize<'de, D: Deserializer<'de>>(
                deserializer: D,
            ) -> Result<jiff::Timestamp, D::Error> {
                $wire::deserialize(deserializer).map(|wire| wire.0)
            }

            /// For an `Option<Timestamp>`: `null` is `None`.
            pub mod option {
                use super::$wire;
                use serde::{Deserialize, Deserializer, Serialize, Serializer};

                pub fn serialize<S: Serializer>(
                    value: &Option<jiff::Timestamp>,
                    serializer: S,
                ) -> Result<S::Ok, S::Error> {
                    value.map($wire).serialize(serializer)
                }

                pub fn deserialize<'de, D: Deserializer<'de>>(
                    deserializer: D,
                ) -> Result<Option<jiff::Timestamp>, D::Error> {
                    Ok(Option::<$wire>::deserialize(deserializer)?.map(|wire| wire.0))
                }
            }

            /// For an `Option<Option<Timestamp>>` on a field with `#[serde(default)]`: an absent field is `None`,
            /// `null` is `Some(None)`.
            pub mod double_option {
                use super::$wire;
                use serde::{Deserialize, Deserializer, Serialize, Serializer};

                pub fn serialize<S: Serializer>(
                    value: &Option<Option<jiff::Timestamp>>,
                    serializer: S,
                ) -> Result<S::Ok, S::Error> {
                    value.flatten().map($wire).serialize(serializer)
                }

                pub fn deserialize<'de, D: Deserializer<'de>>(
                    deserializer: D,
                ) -> Result<Option<Option<jiff::Timestamp>>, D::Error> {
                    Ok(Some(
                        Option::<$wire>::deserialize(deserializer)?.map(|wire| wire.0),
                    ))
                }
            }
        };
    }

    with!(Whole);

    /// The same for a JSON `number`: the fraction of a second is kept.
    pub mod fractional {
        use super::Fraction;
        use serde::{Deserialize, Deserializer, Serialize, Serializer};

        with!(Fraction);
    }
}
#[allow(dead_code)]
pub mod parse_enum_error;
#[allow(dead_code)]
#[doc(hidden)]
pub mod union_dispatch;
pub use self::parse_enum_error::ParseEnumError;
