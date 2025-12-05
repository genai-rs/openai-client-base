#!/usr/bin/env python3
"""
Identifiers of OpenAPI schemas that we intentionally simplify to plain
string types before running the Rust generator.

Keep this list in sync with the spec patching pipeline. These schemas get
flattened in scripts/fix_model_fields.py, and downstream scripts rely on the
tracking file emitted there to clean up generated enums.
"""

from typing import Set

SIMPLIFIED_SCHEMAS: Set[str] = {
    "ModelIds",
    "ModelIdsShared",
    "ModelIdsResponses",
    "CreateCompletionRequestModel",
    "CreateAssistantRequestModel",
    "CreateThreadAndRunRequestModel",
    "CreateImageRequestModel",
    "CreateEmbeddingRequestModel",
    "CreateSpeechRequestModel",
    "CreateFineTuningJobRequestModel",
    "CreateTranscriptionRequestModel",
    "CreateTranslationRequestModel",
    "CreateModerationRequestModel",
    "CreateImageEditRequestModel",
    "CreateImageVariationRequestModel",
    "VoiceIdsShared",
}
