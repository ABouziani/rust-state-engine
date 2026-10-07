use rust_state_engine::evm::Evm;
use rust_state_engine::memory::MemoryState;
use rust_state_engine::raw::RawState;
use rust_state_engine::state::{Address, Slot, State};

#[test]
fn memory_state_works_with_evm() {
    let address: Address = [1u8; 20];
    let slot: Slot = [2u8; 32];

    let mut state = MemoryState::new();

    state.set_balance(&address, 1000);

    state.set_storage(
        &address,
        &slot,
        &[42u8; 32],
    );

    let mut evm = Evm::new(state);

    let account = evm.account(&address).unwrap();

    assert_eq!(account.account.balance, 1000);
    assert_eq!(account.account.nonce, 0);

    let storage = evm.storage(&address, &slot).unwrap();

    assert_eq!(storage.value[0], 42);

    assert_eq!(evm.balance(&address), Some(1000));

    evm.set_balance(&address, 2000);

    assert_eq!(evm.balance(&address), Some(2000));
}

#[test]
fn raw_state_works_with_evm() {
    let address: Address = [1u8; 20];
    let slot: Slot = [2u8; 32];

    let mut state = RawState::new();

    state.set_balance(&address, 1000);

    state.set_storage(
        &address,
        &slot,
        &[42u8; 32],
    );

    let mut evm = Evm::new(state);

    let account = evm.account(&address).unwrap();

    assert_eq!(account.balance(), 1000);
    assert_eq!(account.nonce(), 0);

    let storage = evm.storage(&address, &slot).unwrap();

    assert_eq!(storage.bytes[0], 42);

    assert_eq!(evm.balance(&address), Some(1000));

    evm.set_balance(&address, 2000);

    assert_eq!(evm.balance(&address), Some(2000));
}

#[test]
fn missing_account_returns_none() {
    let state = MemoryState::new();

    let evm = Evm::new(state);

    let address: Address = [9u8; 20];

    assert!(evm.account(&address).is_none());
    assert_eq!(evm.balance(&address), None);
}

#[test]
fn missing_storage_returns_none() {
    let state = MemoryState::new();

    let evm = Evm::new(state);

    let address: Address = [1u8; 20];
    let slot: Slot = [2u8; 32];

    assert!(evm.storage(&address, &slot).is_none());
}