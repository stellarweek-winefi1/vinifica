#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, symbol_short, Address, Env, Symbol,
};

pub type LotId = u64;

#[derive(Clone)]
pub struct Lot {
    pub issuer: Address,
    pub distribution: Address,
    pub max_supply: i128,
    pub minted_supply: i128,
    pub metadata_hash: Symbol,
}

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WineLotError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    LotNotFound = 3,
    NotAuthorized = 4,
    SupplyExceeded = 5,
    InvalidAmount = 6,
}

#[contract]
pub struct WineLotManager;

const KEY_ADMIN: Symbol = symbol_short!("ADMIN");
const KEY_LOT_SEQ: Symbol = symbol_short!("LOTSEQ");
const LOT_PREFIX: Symbol = symbol_short!("LOT");

fn lot_key(lot_id: LotId) -> (Symbol, LotId) {
    (LOT_PREFIX, lot_id)
}

#[contractimpl]
impl WineLotManager {
    pub fn init(env: Env, admin: Address) -> Result<(), WineLotError> {
        if Self::has_admin(&env) {
            return Err(WineLotError::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&KEY_ADMIN, &admin);
        env.storage().instance().set(&KEY_LOT_SEQ, &0u64);
        Ok(())
    }

    pub fn init_lot(
        env: Env,
        issuer: Address,
        distribution: Address,
        max_supply: i128,
        metadata_hash: Symbol,
    ) -> Result<LotId, WineLotError> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();
        if max_supply <= 0 {
            return Err(WineLotError::InvalidAmount);
        }

        let lot_id = Self::next_lot_id(&env);
        let lot = Lot {
            issuer,
            distribution,
            max_supply,
            minted_supply: 0,
            metadata_hash,
        };

        env.storage()
            .persistent()
            .set(&lot_key(lot_id), &lot_clone(&lot));
        Ok(lot_id)
    }

    pub fn mint_batch(
        env: Env,
        lot_id: LotId,
        amount: i128,
    ) -> Result<i128, WineLotError> {
        if amount <= 0 {
            return Err(WineLotError::InvalidAmount);
        }
        let mut lot = Self::load_lot(&env, lot_id)?;
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        if lot.minted_supply + amount > lot.max_supply {
            return Err(WineLotError::SupplyExceeded);
        }
        lot.minted_supply += amount;
        env.storage()
            .persistent()
            .set(&lot_key(lot_id), &lot_clone(&lot));
        Ok(lot.minted_supply)
    }

    pub fn record_sale(
        env: Env,
        lot_id: LotId,
        amount: i128,
        caller: Address,
    ) -> Result<i128, WineLotError> {
        if amount <= 0 {
            return Err(WineLotError::InvalidAmount);
        }
        caller.require_auth();
        let lot = Self::load_lot(&env, lot_id)?;
        if caller != lot.distribution && caller != Self::require_admin(&env)? {
            return Err(WineLotError::NotAuthorized);
        }
        if amount > lot.minted_supply {
            return Err(WineLotError::SupplyExceeded);
        }
        Ok(lot.minted_supply - amount)
    }

    pub fn get_lot(env: Env, lot_id: LotId) -> Result<Lot, WineLotError> {
        Self::load_lot(&env, lot_id)
    }

    pub fn get_admin(env: Env) -> Option<Address> {
        env.storage().instance().get(&KEY_ADMIN)
    }

    fn load_lot(env: &Env, lot_id: LotId) -> Result<Lot, WineLotError> {
        env.storage()
            .persistent()
            .get(&lot_key(lot_id))
            .ok_or(WineLotError::LotNotFound)
    }

    fn has_admin(env: &Env) -> bool {
        env.storage().instance().has(&KEY_ADMIN)
    }

    fn require_admin(env: &Env) -> Result<Address, WineLotError> {
        env.storage()
            .instance()
            .get(&KEY_ADMIN)
            .ok_or(WineLotError::NotInitialized)
    }

    fn next_lot_id(env: &Env) -> LotId {
        let mut seq: LotId = env.storage().instance().get(&KEY_LOT_SEQ).unwrap();
        seq += 1;
        env.storage().instance().set(&KEY_LOT_SEQ, &seq);
        seq
    }
}

fn lot_clone(lot: &Lot) -> Lot {
    Lot {
        issuer: lot.issuer.clone(),
        distribution: lot.distribution.clone(),
        max_supply: lot.max_supply,
        minted_supply: lot.minted_supply,
        metadata_hash: lot.metadata_hash,
    }
}

#[cfg(test)]
mod test;
