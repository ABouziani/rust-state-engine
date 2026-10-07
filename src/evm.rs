use crate::state::{Address, Slot, State};

pub struct Evm<S: State> {
    pub state: S,
}

impl<S: State> Evm<S> {
    pub fn new(state: S) -> Self {
        Self { state }
    }

    pub fn account<'a>(
        &'a self,
        address: &Address,
    ) -> Option<S::Account<'a>> {
        self.state.account(address)
    }

    pub fn storage<'a>(
        &'a self,
        address: &Address,
        slot: &Slot,
    ) -> Option<S::Storage<'a>> {
        self.state.storage(address, slot)
    }

    pub fn balance(
        &self,
        address: &Address,
    ) -> Option<u64> {
        self.state.balance(address)
    }

    pub fn set_balance(
        &mut self,
        address: &Address,
        balance: u64,
    ) {
        self.state.set_balance(address, balance);
    }

    pub fn set_storage(
        &mut self,
        address: &Address,
        slot: &Slot,
        value: &[u8],
    ) {
        self.state.set_storage(address, slot, value);
    }
}