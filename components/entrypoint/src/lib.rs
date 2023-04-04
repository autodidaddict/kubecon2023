wit_bindgen::generate!({
    path: "../wit",
    world: "entrypoint"
});

mod types;

use crate::aggregate::{apply_event, EventWithState};
use crate::concordance_types::StateAck;
use crate::types::{EventWithStateDef, StateAckDef};

const APPLY_EVENT: &str = "AggregateService.ApplyEvent";

/// This component exports the actor interface (guest_call) to the host runtime and
/// imports the generalized concordance aggregate interface's `apply_event`
struct Entrypoint;

impl actor::Actor for Entrypoint {
    fn guest_call(operation: String, payload: Option<Vec<u8>>) -> Result<Option<Vec<u8>>, String> {
        let Some(payload) = payload else {
            return Err("Cannot perform operation with empty payload".to_string())
        };
        match operation.as_str() {
            APPLY_EVENT => {
                eprintln!("Applying event");
                let ews: EventWithState =
                    wasmbus_rpc::common::deserialize::<EventWithStateDef>(&payload)
                        .unwrap_or_default()
                        .into();

                // use the concordance ES interface to apply the event
                let result: StateAck = apply_event(&ews)
                    .map_err(|e| format!("[error][entrypoint] failed to apply event: {e}"))?;
                // Serialize resulting ACK
                let bytes = wasmbus_rpc::common::serialize::<StateAckDef>(&result.into())
                    .map_err(|e| format!("[error][entrypoint] failed to serialize: {e}"))?;

                Ok(Some(bytes))
            }
            op => Err(format!("[error][entrypoint] unrecognized operation [{op}]")),
        }
    }
}

export_entrypoint!(Entrypoint);
