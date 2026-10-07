use std::collections::HashMap;
use std::convert::TryInto;

use crate::state::{Address, Slot, State};

pub struct RawAccount<'a> {
    pub bytes: &'a [u8],
}

impl<'a> RawAccount<'a> {
    pub fn balance(&self) -> u64 {
        u64::from_le_bytes(
            self.bytes[0..8]
                .try_into()
                .expect("account balance requires 8 bytes"),
        )
    }

    pub fn nonce(&self) -> u64 {
        u64::from_le_bytes(
            self.bytes[8..16]
                .try_into()
                .expect("account nonce requires 8 bytes"),
        )
    }
}

pub struct RawStorage<'a> {
    pub bytes: &'a [u8],
}

impl<'a> RawStorage<'a> {
    pub fn value(&self) -> &[u8] {
        self.bytes
    }
}

pub struct RawState {
    pub accounts: HashMap<Address, Vec<u8>>,
    pub storage: HashMap<(Address, Slot), Vec<u8>>,
}

impl RawState {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            storage: HashMap::new(),
        }
    }
}

impl Default for RawState {
    fn default() -> Self {
        Self::new()
    }
}

impl State for RawState {
    type Account<'a> = RawAccount<'a>
    where
        Self: 'a;

    type Storage<'a> = RawStorage<'a>
    where
        Self: 'a;

    fn account<'a>(
        &'a self,
        address: &Address,
    ) -> Option<Self::Account<'a>> {
        self.accounts
            .get(address)
            .map(|bytes| RawAccount { bytes })
    }

    fn storage<'a>(
        &'a self,
        address: &Address,
        slot: &Slot,
    ) -> Option<Self::Storage<'a>> {
        self.storage
            .get(&(*address, *slot))
            .map(|bytes| RawStorage { bytes })
    }

    fn balance(&self, address: &Address) -> Option<u64> {
        self.accounts.get(address).map(|bytes| {
            u64::from_le_bytes(
                bytes[0..8]
                    .try_into()
                    .expect("account balance requires 8 bytes"),
            )
        })
    }

    fn set_balance(
        &mut self,
        address: &Address,
        balance: u64,
    ) {
        let bytes = self
            .accounts
            .entry(*address)
            .or_insert_with(|| vec![0u8; 16]);

        bytes[0..8].copy_from_slice(&balance.to_le_bytes());
    }

    fn set_storage(
        &mut self,
        address: &Address,
        slot: &Slot,
        value: &[u8],
    ) {
        self.storage
            .insert((*address, *slot), value.to_vec());
    }
}