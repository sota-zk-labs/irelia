use aptos_sdk::rest_client::aptos_api_types::{MoveType, VersionedEvent};
use aptos_sdk::rest_client::Client;
use aptos_sdk::types::account_address::AccountAddress;

pub struct EventTracker {
    client: Client,
    account_address: AccountAddress,
    creation_number: usize,
    typ: MoveType,
}

impl EventTracker {
    pub fn new(
        client: Client,
        account_address: AccountAddress,
        typ: MoveType,
        creation_number: usize,
    ) -> Self {
        Self {
            client,
            account_address,
            typ,
            creation_number,
        }
    }
}

impl EventTracker {
    pub async fn latest_event(&mut self) -> Option<VersionedEvent> {
        let mut result = None;
        loop {
            eprintln!("self.creation_number = {:#?}", self.creation_number);
            let creation_number = self.creation_number + 1;
            let events = self.client.get_account_events(
                self.account_address,
                &creation_number.to_string(),
                None,
                None,
            ).await.unwrap().into_inner();
            if events.len() == 0 { break; };
            self.creation_number = creation_number;
            events.into_iter().for_each(|e| {
                eprintln!("e.typ.to_string() = {:#?}", e.typ.to_string());
                if e.typ == self.typ {
                    result = Some(e);
                }
            })
        }
        result
    }
}