use zephyr_sdk::{ prelude::*, soroban_sdk::{xdr::{ ContractEvent,  ContractEventBody, ScVal}, Symbol}, EnvClient };

use crate::PairsTable;

use crate::factory::types;

pub(crate) fn get_event_from_new_pair(env: &EnvClient, data: &ScVal) -> PairsTable {
    let values: types::NewPairEvent = env.from_scval(data);
                
    let table = PairsTable {
        token_a: env.to_scval(values.token_0.clone()),
        token_b: env.to_scval(values.token_1.clone() ),
        address: env.to_scval(values.pair.clone()),
    };

    table
}

pub(crate) fn handle_contract_events(env: &EnvClient, contract_events: Vec<ContractEvent>) {
    for event in contract_events {
        let ContractEventBody::V0(event) = &event.body;

        let action: Symbol = env.from_scval(&event.topics[1]);

        let data = &event.data;

        if action == Symbol::new(&env.soroban(), "new_pair") {
            env.log().debug(
                format!(
                    "Event captured: new_pair"
                ),
                None,
            );
            
            let table: PairsTable = get_event_from_new_pair(&env, data);

            table.put(&env);
        }
    }
}