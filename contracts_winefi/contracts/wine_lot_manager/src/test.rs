#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env};

fn setup() -> (Env, Address, Address, Address) {
    let env = Env::default();
    let admin = Address::generate(&env);
    let issuer = Address::generate(&env);
    let distribution = Address::generate(&env);
    (env, admin, issuer, distribution)
}

#[test]
fn test_init_and_lot_flow() {
    let (env, admin, issuer, distribution) = setup();
    WineLotManager::init(env.clone(), admin.clone()).unwrap();

    let lot_id = WineLotManager::init_lot(
        env.clone(),
        issuer.clone(),
        distribution.clone(),
        1_000,
        symbol_short!("META"),
    )
    .unwrap();

    let lot = WineLotManager::get_lot(env.clone(), lot_id).unwrap();
    assert_eq!(lot.max_supply, 1_000);

    WineLotManager::mint_batch(env.clone(), lot_id, 500).unwrap();
    let updated = WineLotManager::get_lot(env.clone(), lot_id).unwrap();
    assert_eq!(updated.minted_supply, 500);
}
