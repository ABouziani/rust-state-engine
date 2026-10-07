use std::collections::HashMap;

use crate::state::{Address, Slot, State};

pub struct Overlay<S: State> {
    pub base: S,
    pub balances: HashMap<Address, u64>,
    pub storage: HashMap<(Address, Slot), Vec<u8>>,
}

impl<S: State> Overlay<S> {
    pub fn new(base: S) -> Self {
        Self {
            base,
            balances: HashMap::new(),
            storage: HashMap::new(),
        }
    }

    pub fn balance(
        &self,
        address: &Address,
    ) -> Option<u64> {
        if let Some(balance) = self.balances.get(address) {
            return Some(*balance);
        }

        self.base.balance(address)
    }

    pub fn set_balance(
        &mut self,
        address: &Address,
        balance: u64,
    ) {
        self.balances.insert(*address, balance);
    }

    pub fn storage(
        &self,
        address: &Address,
        slot: &Slot,
    ) -> Option<&[u8]> {
        if let Some(value) = self.storage.get(&(*address, *slot)) {
            return Some(value.as_slice());
        }

        None
    }

    pub fn set_storage(
        &mut self,
        address: &Address,
        slot: &Slot,
        value: &[u8],
    ) {
        self.storage
            .insert((*address, *slot), value.to_vec());
    }

    pub fn commit(
        &mut self,
    ) {
        for (address, balance) in self.balances.drain() {
            self.base.set_balance(&address, balance);
        }

        for ((address, slot), value) in self.storage.drain() {
            self.base.set_storage(
                &address,
                &slot,
                &value,
            );
        }
    }

    pub fn rollback(
        &mut self,
    ) {
        self.balances.clear();
        self.storage.clear();
    }

    pub fn has_changes(&self) -> bool {
        !self.balances.is_empty()
            || !self.storage.is_empty()
    }
}