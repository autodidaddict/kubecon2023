wit_bindgen::generate!({
    path: "../wit",
    world: "bankaccount-aggregate"
});

use crate::bankaccount_types::AccountCreatedEvent;
use account_aggregate::{AccountAggregate, AggregateState};

struct BankAccount;

impl AccountAggregate for BankAccount {
    fn apply_account_created(
        _state: AggregateState,
        event: AccountCreatedEvent,
    ) -> Result<AggregateState, String> {
        println!("{}", event.account_number);
        todo!()
    }
}

export_bankaccount_aggregate!(BankAccount);
