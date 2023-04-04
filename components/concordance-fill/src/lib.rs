use aggregate::Aggregate;
use concordance_types::{EventWithState, StateAck};
use serde::{Deserialize, Serialize};

use crate::{
    bankaccount::apply_account_created,
    bankaccount_types::{AccountCreatedEvent, AggregateState},
};

wit_bindgen::generate!({
    path: "../wit",
    world: "concordance"
});

struct AggFill;

impl Aggregate for AggFill {
    fn apply_event(event: EventWithState) -> Result<StateAck, String> {
        if event.event.event_type != "account_created" {
            return Err(format!(
                "Unsupported event type: {}",
                event.event.event_type
            ));
        }

        // here we're taking advantage of hidden knowledge that the aggregate's state from the
        // "other" (wasmbus-based actor) is serialized as JSON
        let res = apply_account_created(
            None,
            &AccountCreatedEvent {
                initial_balance: 300,
                account_number: "ABC123".to_string(),
                min_balance: 300,
                customer_id: "CUSTBOB".to_string(),
            },
        )?;

        let res: AggregateStateDef = res.into();
        Ok(StateAck {
            succeeded: true,
            state: Some(serde_json::to_vec(&res).unwrap()),
            error: None,
        })
    }
}

export_concordance_aggregate!(AggFill);

/// Copied from the "other" aggregate actor
#[derive(Serialize, Deserialize, Clone, Default)]
struct AggregateStateDef {
    // cents to avoid using float
    pub balance: u32,
    pub min_balance: u32,
    pub reserved_amount: u32,
    pub account_number: String,
    pub customer_id: String,
}

impl From<AggregateState> for AggregateStateDef {
    fn from(source: AggregateState) -> Self {
        AggregateStateDef {
            balance: source.balance,
            min_balance: source.min_balance,
            reserved_amount: source.reserved_amount,
            account_number: source.account_number,
            customer_id: source.customer_id,
        }
    }
}
