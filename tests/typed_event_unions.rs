//! The unions that used to be `serde_json::Value` (`jsonValueSchemas`): response output items, computer actions, and
//! the streaming and realtime event unions. Each decodes by its `type` and serializes back to the same JSON.

use openai_client_base::models;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

fn round_trips<T: DeserializeOwned + Serialize>(value: Value) -> T {
    let decoded: T =
        serde_json::from_value(value.clone()).unwrap_or_else(|e| panic!("{value}: {e}"));
    assert_eq!(serde_json::to_value(&decoded).unwrap(), value);
    decoded
}

/// What `POST /responses` with `stream: true` sends, one `data:` payload at a time.
#[test]
fn response_stream_events_decode_by_type() {
    let added: models::ResponseStreamEvent = round_trips(json!({
        "type": "response.output_item.added",
        "output_index": 0,
        "sequence_number": 1,
        "item": {
            "type": "function_call",
            "id": "fc_1",
            "call_id": "call_1",
            "name": "get_weather",
            "arguments": "",
            "status": "in_progress"
        }
    }));
    match added {
        models::ResponseStreamEvent::ResponseOutputItemAddedEvent(event) => match event.item {
            models::OutputItem::FunctionToolCall(call) => assert_eq!(call.name, "get_weather"),
            other => panic!("{other:?}"),
        },
        other => panic!("{other:?}"),
    }

    let delta: models::ResponseStreamEvent = round_trips(json!({
        "type": "response.output_text.delta",
        "item_id": "msg_1",
        "output_index": 0,
        "content_index": 0,
        "delta": "Hel",
        "sequence_number": 2,
        "logprobs": []
    }));
    assert!(
        matches!(delta, models::ResponseStreamEvent::ResponseTextDeltaEvent(d) if d.delta == "Hel")
    );

    // An event type added after this client was generated does not fail the stream.
    let new: models::ResponseStreamEvent =
        round_trips(json!({"type": "response.something.new", "sequence_number": 3}));
    assert!(matches!(new, models::ResponseStreamEvent::Unknown(_)));
}

#[test]
fn a_computer_call_has_typed_actions() {
    let item: models::OutputItem = round_trips(json!({
        "type": "computer_call",
        "id": "cu_1",
        "call_id": "call_1",
        "action": {"type": "click", "button": "left", "x": 10, "y": 20},
        "actions": [{"type": "wait"}, {"type": "zoom", "factor": 2}],
        "pending_safety_checks": [],
        "status": "completed"
    }));
    let models::OutputItem::ComputerToolCall(call) = item else {
        panic!("not a computer call")
    };
    assert!(
        matches!(call.action, Some(models::ComputerAction::ClickParam(ref c)) if c.x == 10 && c.y == 20)
    );
    let actions = call.actions.unwrap();
    assert!(matches!(actions[0], models::ComputerAction::WaitParam(_)));
    assert!(matches!(actions[1], models::ComputerAction::Unknown(_)));
}

#[test]
fn transcription_stream_events_decode_by_type() {
    let delta: models::CreateTranscriptionResponseStreamEvent =
        round_trips(json!({"type": "transcript.text.delta", "delta": "hello"}));
    assert!(matches!(
        delta,
        models::CreateTranscriptionResponseStreamEvent::TranscriptTextDeltaEvent(d) if d.delta == "hello"
    ));
}

/// Realtime events travel over a WebSocket, which no operation describes, so they have no catch-all.
#[test]
fn realtime_events_are_typed() {
    let error: models::RealtimeServerEvent = round_trips(json!({
        "type": "error",
        "event_id": "event_1",
        "error": {"type": "invalid_request_error", "message": "bad"}
    }));
    assert!(matches!(
        error,
        models::RealtimeServerEvent::RealtimeServerEventError(e) if e.error.message == "bad"
    ));
    let unknown = serde_json::from_value::<models::RealtimeServerEvent>(
        json!({"type": "brand.new", "event_id": "e"}),
    );
    assert!(unknown.unwrap_err().to_string().contains("brand.new"));
}
