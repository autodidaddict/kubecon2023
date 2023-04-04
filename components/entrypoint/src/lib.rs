wit_bindgen::generate!({
    path: "../wit",
    world: "entrypoint"
});

use serde::{Serialize, Deserialize};

use crate::aggregate::{apply_event, EventWithState};
use crate::concordance_types::{StateAck, ConcordanceEvent};
use wasmbus_rpc::common::{deserialize, serialize};

struct Entrypoint;

#[derive(Serialize, Deserialize)]
struct StateAckDef {
    state: Option<Vec<u8>>,
    succeeded: bool,
    error: Option<String>,
}

impl From<StateAckDef> for StateAck {
    fn from(StateAckDef { state, succeeded, error }: StateAckDef) -> StateAck {
        StateAck { state, succeeded, error }
    }
}

impl From<StateAck> for StateAckDef {
    fn from(StateAck { state, succeeded, error }: StateAck) -> StateAckDef { 
        StateAckDef { state, succeeded, error }
    }
}

#[derive(Serialize, Deserialize)]
struct ConcordanceEventDef {
    stream_id: String,
    event_type: String,
    payload: Option<Vec<u8>>,
}

impl From<ConcordanceEventDef> for ConcordanceEvent {
    fn from(ConcordanceEventDef { stream_id, event_type, payload }: ConcordanceEventDef) -> ConcordanceEvent {
        ConcordanceEvent { stream_id, event_type, payload }
    }
}

impl From<ConcordanceEvent> for ConcordanceEventDef {
    fn from(ConcordanceEvent { stream_id, event_type, payload }: ConcordanceEvent) -> ConcordanceEventDef {
        ConcordanceEventDef { stream_id, event_type, payload }
    }
}

#[derive(Serialize, Deserialize)]
struct EventWithStateDef {
    event: ConcordanceEventDef,
    state: Option<Vec<u8>>,
}

impl From<EventWithState> for EventWithStateDef {
    fn from(EventWithState { event, state }: EventWithState) -> EventWithStateDef {
        EventWithStateDef { event: event.into(), state }
    }
}

impl From<EventWithStateDef> for EventWithState {
    fn from(EventWithStateDef { event, state }: EventWithStateDef) -> EventWithState {
        EventWithState { event: event.into(), state }
    }
}

impl actor::Actor for Entrypoint {
    fn guest_call(
        operation: String,
        payload: Option<Vec<u8>>,
    ) -> Result<Option<Vec<u8>>, String> {
        match operation.as_str() {
            "Entrypoint.Run" => {
                eprintln!("[info][entrypoint] applying event");

                // Deserialize the event
                let ews: EventWithState = match payload {
                    Some(p1) => match deserialize::<EventWithStateDef>(&p1) {
                        // If event successfully parsed
                        Ok(event_def) => event_def.into(),

                        // If event failed to be parsed, use a stub
                        Err(err) => {
                            eprintln!("[error][entrypoint] failed to deserialize event: {err}");
                            EventWithState {
                                event: ConcordanceEvent {
                                    stream_id: "unknown".into(),
                                    event_type: "stub".into(),
                                    payload: None,
                                },
                                state: None,
                            }
                        },
                    },

                    // No payload was provided
                    None => {
                        EventWithState {
                            event: ConcordanceEvent {
                                stream_id: "unknown".into(),
                                event_type: "stub".into(),
                                payload: None,
                            },
                            state: None,
                        }
                    }
                };

                // Apply the event
                let result: StateAck = apply_event(&ews)
                    .map_err(|e| format!("[error][entrypoint] failed to apply event: {e}"))?;

                // Serialize resulting ACK
                let bytes = serialize::<StateAckDef>(&result.into())
                    .map_err(|e| format!("[error][entrypoint] failed to serialize: {e}"))?;

                Ok(Some(bytes))
            }
            op => Err(format!("[error][entrypoint] unrecognized operation [{op}]")),
        }
    }
}

export_entrypoint!(Entrypoint);
