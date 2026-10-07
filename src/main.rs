use rust_state_engine::memory::MemoryState;
use rust_state_engine::overlay::Overlay;
use rust_state_engine::state::{Address, State};

fn main() {
    let address: Address = [1u8; 20];

    let mut state = MemoryState::new();

    state.set_balance(&address, 1000);

    println!(
        "Base balance: {}",
        state.balance(&address).unwrap()
    );

    let mut overlay = Overlay::new(state);

    overlay.set_balance(&address, 2000);

    println!(
        "Overlay balance: {}",
        overlay.balance(&address).unwrap()
    );

    println!(
        "Base balance before commit: {}",
        overlay.base.balance(&address).unwrap()
    );

    overlay.commit();

    println!(
        "Base balance after commit: {}",
        overlay.base.balance(&address).unwrap()
    );
}