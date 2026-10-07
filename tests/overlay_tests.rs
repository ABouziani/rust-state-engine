use rust_state_engine::memory::MemoryState;
use rust_state_engine::overlay::Overlay;
use rust_state_engine::state::{Address, Slot, State};

fn create_base_state() -> MemoryState {
    let address: Address = [1u8; 20];
    let slot: Slot = [2u8; 32];

    let mut state = MemoryState::new();

    state.set_balance(&address, 1000);

    state.set_storage(
        &address,
        &slot,
        &[42u8; 32],
    );

    state
}

#[test]
fn overlay_reads_from_base() {
    let address: Address = [1u8; 20];

    let state = create_base_state();

    let overlay = Overlay::new(state);

    assert_eq!(overlay.balance(&address), Some(1000));
}

#[test]
fn overlay_write_does_not_modify_base() {
    let address: Address = [1u8; 20];

    let state = create_base_state();

    let mut overlay = Overlay::new(state);

    overlay.set_balance(&address, 2000);

    assert_eq!(overlay.balance(&address), Some(2000));

    assert_eq!(overlay.base.balance(&address), Some(1000));
}

#[test]
fn overlay_commit_updates_base() {
    let address: Address = [1u8; 20];

    let state = create_base_state();

    let mut overlay = Overlay::new(state);

    overlay.set_balance(&address, 2000);

    assert_eq!(overlay.base.balance(&address), Some(1000));

    overlay.commit();

    assert_eq!(overlay.base.balance(&address), Some(2000));

    assert!(!overlay.has_changes());
}

#[test]
fn overlay_rollback_discards_changes() {
    let address: Address = [1u8; 20];

    let state = create_base_state();

    let mut overlay = Overlay::new(state);

    overlay.set_balance(&address, 5000);

    assert_eq!(overlay.balance(&address), Some(5000));

    overlay.rollback();

    assert_eq!(overlay.balance(&address), Some(1000));

    assert!(!overlay.has_changes());
}

#[test]
fn overlay_storage_isolated_until_commit() {
    let address: Address = [1u8; 20];
    let slot: Slot = [2u8; 32];

    let state = create_base_state();

    let mut overlay = Overlay::new(state);

    overlay.set_storage(
        &address,
        &slot,
        &[99u8; 32],
    );

    assert_eq!(
        overlay.storage(&address, &slot).unwrap()[0],
        99
    );

    assert_eq!(
        overlay.base.storage(&address, &slot).unwrap().value[0],
        42
    );

    overlay.commit();

    assert_eq!(
        overlay.base.storage(&address, &slot).unwrap().value[0],
        99
    );
}