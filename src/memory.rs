use std::collections::HashMap;

use crate::state::{Address, Slot, State, Word};

pub struct Account {
    pub balance: u64,
    pub nonce: u64,
}

pub struct AccountRef<'a> {
    pub account: &'a Account,
}

pub struct StorageRef<'a> {
    pub value: &'a Word,
}

pub struct MemoryState {
    pub accounts: HashMap<Address, Account>,
    pub storage: HashMap<(Address, Slot), Word>,
}

impl MemoryState {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            storage: HashMap::new(),
        }
    }
}

impl Default for MemoryState {
    fn default() -> Self {
        Self::new()
    }
}

impl State for MemoryState {
    type Account<'a> = AccountRef<'a>
    where
        Self: 'a;

    type Storage<'a> = StorageRef<'a>
    where
        Self: 'a;

    fn account<'a>(
        &'a self,
        address: &Address,
    ) -> Option<Self::Account<'a>> {
        self.accounts
            .get(address)
            .map(|account| AccountRef { account })
    }

    fn storage<'a>(
        &'a self,
        address: &Address,
        slot: &Slot,
    ) -> Option<Self::Storage<'a>> {
        self.storage
            .get(&(*address, *slot))
            .map(|value| StorageRef { value })
    }

    fn balance(&self, address: &Address) -> Option<u64> {
        self.accounts.get(address).map(|account| account.balance)
    }

    fn set_balance(
        &mut self,
        address: &Address,
        balance: u64,
    ) {
        self.accounts
            .entry(*address)
            .or_insert(Account {
                balance: 0,
                nonce: 0,
            })
            .balance = balance;
    }

    fn set_storage(
        &mut self,
        address: &Address,
        slot: &Slot,
        value: &[u8],
    ) {
        let mut word = [0u8; 32];

        let len = value.len().min(32);

        word[..len].copy_from_slice(&value[..len]);

        self.storage.insert((*address, *slot), word);
    }
}