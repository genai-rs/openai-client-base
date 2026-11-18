//! Bodies for the common cases, as OpenAI sends them. Each decodes as its operation's model (the tests check it);
//! change a field with `serde_json` (`value["model"] = "...".into()`) before scripting it.

use serde_json::{json, Value};

/// The model the fixtures name.
pub const MODEL: &str = "gpt-4o-mini";
/// The `created` timestamp of the fixtures (2024-01-01T00:00:00Z).
pub const CREATED: i64 = 1_704_067_200;

/// OpenAI's error envelope: `{"error": {"message", "type", "param", "code"}}`.
pub fn error(
    kind: &str,
    message: impl Into<String>,
    param: Option<String>,
    code: Option<String>,
) -> Value {
    json!({
        "error": {
            "message": message.into(),
            "type": kind,
            "param": param,
            "code": code,
        }
    })
}

fn usage() -> Value {
    json!({
        "prompt_tokens": 9,
        "completion_tokens": 12,
        "total_tokens": 21,
    })
}

/// A chat completion (`create_chat_completion`) whose assistant message is `text`.
pub fn chat_completion(text: &str) -> Value {
    json!({
        "id": "chatcmpl-mock",
        "object": "chat.completion",
        "created": CREATED,
        "model": MODEL,
        "choices": [{
            "index": 0,
            "message": {"role": "assistant", "content": text, "refusal": null},
            "finish_reason": "stop",
            "logprobs": null,
        }],
        "usage": usage(),
    })
}

/// A chat completion that calls the function `name` with `arguments` (a JSON object).
pub fn chat_tool_call(name: &str, arguments: Value) -> Value {
    json!({
        "id": "chatcmpl-mock",
        "object": "chat.completion",
        "created": CREATED,
        "model": MODEL,
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": null,
                "refusal": null,
                "tool_calls": [{
                    "id": "call_mock",
                    "type": "function",
                    "function": {"name": name, "arguments": arguments.to_string()},
                }],
            },
            "finish_reason": "tool_calls",
            "logprobs": null,
        }],
        "usage": usage(),
    })
}

fn chat_chunk(delta: Value, finish_reason: Value) -> Value {
    json!({
        "id": "chatcmpl-mock",
        "object": "chat.completion.chunk",
        "created": CREATED,
        "model": MODEL,
        "choices": [{"index": 0, "delta": delta, "finish_reason": finish_reason, "logprobs": null}],
    })
}

/// The chunks of a streamed chat completion: the role, a chunk per piece of `parts`, and the finish.
pub fn chat_chunks(parts: &[&str]) -> Vec<Value> {
    let mut chunks = vec![chat_chunk(
        json!({"role": "assistant", "content": ""}),
        Value::Null,
    )];
    chunks.extend(
        parts
            .iter()
            .map(|part| chat_chunk(json!({"content": part}), Value::Null)),
    );
    chunks.push(chat_chunk(json!({}), json!("stop")));
    chunks
}

fn response_object(status: &str, output: Value) -> Value {
    json!({
        "id": "resp_mock",
        "object": "response",
        "created_at": CREATED,
        "status": status,
        "model": MODEL,
        "output": output,
        "parallel_tool_calls": true,
        "tool_choice": "auto",
        "tools": [],
        "error": null,
        "incomplete_details": null,
        "instructions": null,
        "metadata": {},
        "temperature": 1.0,
        "top_p": 1.0,
        "usage": {
            "input_tokens": 9,
            "input_tokens_details": {"cached_tokens": 0},
            "output_tokens": 12,
            "output_tokens_details": {"reasoning_tokens": 0},
            "total_tokens": 21,
        },
    })
}

fn output_message(text: &str) -> Value {
    json!({
        "type": "message",
        "id": "msg_mock",
        "status": "completed",
        "role": "assistant",
        "content": [{"type": "output_text", "text": text, "annotations": [], "logprobs": []}],
    })
}

/// A response (`create_response`) whose output is the assistant message `text`.
pub fn response(text: &str) -> Value {
    response_object("completed", json!([output_message(text)]))
}

/// A response whose output calls the function `name` with `arguments` (a JSON object).
pub fn response_function_call(name: &str, arguments: Value) -> Value {
    response_object(
        "completed",
        json!([{
            "type": "function_call",
            "id": "fc_mock",
            "call_id": "call_mock",
            "name": name,
            "arguments": arguments.to_string(),
            "status": "completed",
        }]),
    )
}

/// The answer to `delete_response`.
pub fn response_deleted(id: &str) -> Value {
    json!({"id": id, "object": "response", "deleted": true})
}

/// The events of a streamed response: `response.created`, an `output_text.delta` per piece of `parts`,
/// `output_text.done` and `response.completed`.
pub fn response_events(parts: &[&str]) -> Vec<Value> {
    let text: String = parts.concat();
    let mut sequence = 0;
    let mut next = || {
        sequence += 1;
        sequence - 1
    };
    let mut events = vec![json!({
        "type": "response.created",
        "sequence_number": next(),
        "response": response_object("in_progress", json!([])),
    })];
    for part in parts {
        events.push(json!({
            "type": "response.output_text.delta",
            "sequence_number": next(),
            "item_id": "msg_mock",
            "output_index": 0,
            "content_index": 0,
            "delta": part,
            "logprobs": [],
        }));
    }
    events.push(json!({
        "type": "response.output_text.done",
        "sequence_number": next(),
        "item_id": "msg_mock",
        "output_index": 0,
        "content_index": 0,
        "text": text,
        "logprobs": [],
    }));
    events.push(json!({
        "type": "response.completed",
        "sequence_number": next(),
        "response": response(&text),
    }));
    events
}

/// Embeddings (`create_embedding`), one per vector.
pub fn embeddings(vectors: &[&[f32]]) -> Value {
    json!({
        "object": "list",
        "data": vectors.iter().enumerate().map(|(index, vector)| json!({
            "object": "embedding",
            "index": index,
            "embedding": vector,
        })).collect::<Vec<_>>(),
        "model": "text-embedding-3-small",
        "usage": {"prompt_tokens": 8, "total_tokens": 8},
    })
}

/// A model (`retrieve_model`).
pub fn model(id: &str) -> Value {
    json!({"id": id, "object": "model", "created": CREATED, "owned_by": "openai"})
}

/// The list of models (`list_models`).
pub fn models(ids: &[&str]) -> Value {
    json!({"object": "list", "data": ids.iter().map(|id| model(id)).collect::<Vec<_>>()})
}

/// A file (`create_file`, `retrieve_file`).
pub fn file(id: &str, filename: &str, purpose: &str) -> Value {
    json!({
        "id": id,
        "object": "file",
        "bytes": 128,
        "created_at": CREATED,
        "filename": filename,
        "purpose": purpose,
        "status": "processed",
    })
}

/// The list of files (`list_files`).
pub fn files(files: &[Value]) -> Value {
    json!({
        "object": "list",
        "data": files,
        "first_id": files.first().and_then(|f| f.get("id")).cloned(),
        "last_id": files.last().and_then(|f| f.get("id")).cloned(),
        "has_more": false,
    })
}

/// The answer to `delete_file`.
pub fn file_deleted(id: &str) -> Value {
    json!({"id": id, "object": "file", "deleted": true})
}

/// A batch (`create_batch`, `retrieve_batch`, `cancel_batch`) in `status` (`validating`, `in_progress`,
/// `completed`, `cancelled`, ...).
pub fn batch(id: &str, status: &str) -> Value {
    json!({
        "id": id,
        "object": "batch",
        "endpoint": "/v1/chat/completions",
        "input_file_id": "file-mock",
        "completion_window": "24h",
        "status": status,
        "created_at": CREATED,
        "request_counts": {"total": 1, "completed": 0, "failed": 0},
        "metadata": {},
    })
}

/// The list of batches (`list_batches`).
pub fn batches(batches: &[Value]) -> Value {
    json!({
        "object": "list",
        "data": batches,
        "first_id": batches.first().and_then(|b| b.get("id")).cloned(),
        "last_id": batches.last().and_then(|b| b.get("id")).cloned(),
        "has_more": false,
    })
}

/// Generated or edited images (`create_image`, `create_image_edit`), one per base64 PNG.
pub fn images(b64_json: &[&str]) -> Value {
    json!({
        "created": CREATED,
        "data": b64_json.iter().map(|b| json!({"b64_json": b})).collect::<Vec<_>>(),
    })
}

/// A transcription in `json` format (`create_transcription`).
pub fn transcription(text: &str) -> Value {
    json!({"text": text})
}

/// A moderation (`create_moderation`) of one input, flagged or not.
pub fn moderation(flagged: bool) -> Value {
    let categories = [
        "harassment",
        "harassment/threatening",
        "hate",
        "hate/threatening",
        "illicit",
        "illicit/violent",
        "self-harm",
        "self-harm/instructions",
        "self-harm/intent",
        "sexual",
        "sexual/minors",
        "violence",
        "violence/graphic",
    ];
    let flags: serde_json::Map<String, Value> = categories
        .iter()
        .map(|c| ((*c).to_owned(), json!(flagged)))
        .collect();
    let scores: serde_json::Map<String, Value> = categories
        .iter()
        .map(|c| ((*c).to_owned(), json!(if flagged { 0.9 } else { 0.01 })))
        .collect();
    let applied: serde_json::Map<String, Value> = categories
        .iter()
        .map(|c| ((*c).to_owned(), json!(["text"])))
        .collect();
    json!({
        "id": "modr-mock",
        "model": "omni-moderation-latest",
        "results": [{
            "flagged": flagged,
            "categories": flags,
            "category_scores": scores,
            "category_applied_input_types": applied,
        }],
    })
}

/// An SDP answer for `create_realtime_call` (see [`Reply::sdp`](crate::Reply::sdp)).
pub const SDP_ANSWER: &str =
    "v=0\r\no=- 0 0 IN IP4 127.0.0.1\r\ns=-\r\nt=0 0\r\nm=audio 9 UDP/TLS/RTP/SAVPF 111\r\n";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models;
    use serde::de::DeserializeOwned;

    fn decodes<T: DeserializeOwned>(value: Value) {
        if let Err(e) = serde_json::from_value::<T>(value.clone()) {
            panic!(
                "{} does not decode {value}: {e}",
                std::any::type_name::<T>()
            );
        }
    }

    #[test]
    fn every_fixture_decodes_as_its_model() {
        decodes::<models::CreateChatCompletionResponse>(chat_completion("hi"));
        decodes::<models::CreateChatCompletionResponse>(chat_tool_call(
            "get_weather",
            json!({"city": "Ghent"}),
        ));
        for chunk in chat_chunks(&["Hel", "lo"]) {
            decodes::<models::CreateChatCompletionStreamResponse>(chunk);
        }
        decodes::<models::Response>(response("hi"));
        decodes::<models::Response>(response_function_call(
            "get_weather",
            json!({"city": "Ghent"}),
        ));
        for event in response_events(&["Hel", "lo"]) {
            decodes::<models::ResponseStreamEvent>(event);
        }
        decodes::<models::DeleteResponse200Response>(response_deleted("resp_mock"));
        decodes::<models::CreateEmbeddingResponse>(embeddings(&[&[0.1, 0.2]]));
        decodes::<models::Model>(model("gpt-4o"));
        decodes::<models::ListModelsResponse>(models(&["gpt-4o", "gpt-4o-mini"]));
        decodes::<models::OpenAiFile>(file("file-1", "data.jsonl", "batch"));
        decodes::<models::ListFilesResponse>(files(&[file("file-1", "data.jsonl", "batch")]));
        decodes::<models::DeleteFileResponse>(file_deleted("file-1"));
        decodes::<models::Batch>(batch("batch_1", "in_progress"));
        decodes::<models::ListBatchesResponse>(batches(&[batch("batch_1", "completed")]));
        decodes::<models::ImagesResponse>(images(&["aGVsbG8="]));
        decodes::<models::CreateTranscription200Response>(transcription("hello"));
        decodes::<models::CreateModerationResponse>(moderation(true));
        decodes::<models::ErrorResponse>(error(
            "invalid_request_error",
            "bad",
            Some("model".into()),
            None,
        ));
    }
}
