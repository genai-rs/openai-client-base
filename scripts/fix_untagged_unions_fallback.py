"""
Fallback hardcoded types for when OpenAPI spec is not available.
This file contains the known untagged unions as of the last manual update.
"""

# Map of types that should be untagged enums with their variants
UNTAGGED_UNIONS = {
    'ChatCompletionRequestSystemMessageContent': [
        ('Text', 'String'),
        ('Array', 'Vec<models::ChatCompletionRequestMessageContentPartText>'),
    ],
    'ChatCompletionRequestUserMessageContent': [
        ('Text', 'String'),
        ('Array', 'Vec<models::ChatCompletionRequestUserMessageContentPart>'),
    ],
    'ChatCompletionRequestAssistantMessageContent': [
        ('Text', 'String'),
        ('Array', 'Vec<models::ChatCompletionRequestAssistantMessageContentPart>'),
    ],
    'ChatCompletionRequestToolMessageContent': [
        ('Text', 'String'),
        ('Array', 'Vec<models::ChatCompletionRequestMessageContentPartText>'),
    ],
    'ChatCompletionRequestDeveloperMessageContent': [
        ('Text', 'String'),
        ('Array', 'Vec<models::ChatCompletionRequestMessageContentPartText>'),
    ],
    'ChatCompletionRequestFunctionMessageContent': [
        ('Text', 'String'),
        ('Null', '()'),  # null type for function messages
    ],
    'ChatCompletionToolChoiceOption': [
        ('Auto', 'ChatCompletionToolChoiceOptionAuto'),
        ('Named', 'models::ChatCompletionNamedToolChoice'),
    ],
    'CreateEmbeddingRequestInput': [
        ('Text', 'String'),
        ('ArrayOfStrings', 'Vec<String>'),
        ('ArrayOfIntegers', 'Vec<i32>'),
        ('ArrayOfIntegerArrays', 'Vec<Vec<i32>>'),
    ],
    'AssistantsApiToolChoiceOption': [
        ('Auto', 'AssistantsApiToolChoiceOptionAuto'),
        ('Named', 'models::AssistantsNamedToolChoice'),
    ],
    'AssistantsApiResponseFormatOption': [
        ('Auto', 'AssistantsApiResponseFormatOptionAuto'),
        ('Text', 'AssistantsApiResponseFormatOptionText'),
        ('JsonObject', 'models::ResponseFormatJsonObject'),
        ('JsonSchema', 'models::ResponseFormatJsonSchema'),
    ],
}

# Simple string literals that can be enum variants
SIMPLE_STRING_ENUMS = {
    'ChatCompletionToolChoiceOptionAuto': ['none', 'auto', 'required'],
    'AssistantsApiToolChoiceOptionAuto': ['none', 'auto', 'required'],
    'AssistantsApiResponseFormatOptionAuto': ['auto'],
    'AssistantsApiResponseFormatOptionText': ['text'],
}