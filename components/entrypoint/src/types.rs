//! the sole reason this module is necessary is because wit-bindgen doesn't put
//! default or serialize or deserialize on its generated structs

use serde::{Deserialize, Serialize};

use crate::concordance_types::{ConcordanceEvent, EventWithState, StateAck};

#[derive(Serialize, Deserialize, Default)]
pub struct StateAckDef {
    state: Option<Vec<u8>>,
    succeeded: bool,
    error: Option<String>,
}

impl From<StateAckDef> for StateAck {
    fn from(
        StateAckDef {
            state,
            succeeded,
            error,
        }: StateAckDef,
    ) -> StateAck {
        StateAck {
            state,
            succeeded,
            error,
        }
    }
}

impl From<StateAck> for StateAckDef {
    fn from(
        StateAck {
            state,
            succeeded,
            error,
        }: StateAck,
    ) -> StateAckDef {
        StateAckDef {
            state,
            succeeded,
            error,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct EventWithStateDef {
    event: ConcordanceEventDef,
    state: Option<Vec<u8>>,
}

impl From<EventWithState> for EventWithStateDef {
    fn from(EventWithState { event, state }: EventWithState) -> EventWithStateDef {
        EventWithStateDef {
            event: event.into(),
            state,
        }
    }
}

impl From<EventWithStateDef> for EventWithState {
    fn from(EventWithStateDef { event, state }: EventWithStateDef) -> EventWithState {
        EventWithState {
            event: event.into(),
            state,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ConcordanceEventDef {
    stream_id: String,
    event_type: String,
    payload: Option<Vec<u8>>,
}

impl From<ConcordanceEventDef> for ConcordanceEvent {
    fn from(
        ConcordanceEventDef {
            stream_id,
            event_type,
            payload,
        }: ConcordanceEventDef,
    ) -> ConcordanceEvent {
        ConcordanceEvent {
            stream_id,
            event_type,
            payload,
        }
    }
}

impl From<ConcordanceEvent> for ConcordanceEventDef {
    fn from(
        ConcordanceEvent {
            stream_id,
            event_type,
            payload,
        }: ConcordanceEvent,
    ) -> ConcordanceEventDef {
        ConcordanceEventDef {
            stream_id,
            event_type,
            payload,
        }
    }
}
