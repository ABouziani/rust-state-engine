# Rust State Engine

A generic, zero-copy blockchain state engine written in Rust.

This project explores the architecture and Rust patterns behind high-performance blockchain execution and state management systems.

The main focus is on **trait-driven architecture, Generic Associated Types (GATs), lifetimes, zero-copy data access, pluggable state backends, and transactional state overlays**.

---

## Features

* Generic `State` abstraction
* Generic Associated Types (GATs)
* Lifetime-aware borrowed views
* Zero-copy account access
* Multiple state representations
* Generic EVM execution layer
* Transaction state overlays
* Commit / rollback semantics
* In-memory state backend
* Raw byte-oriented state backend
* Unit and integration tests
* Designed for future persistent storage backends

---

## Architecture

```text
                         EVM
                          |
                          v
                  +---------------+
                  |     State     |
                  |     Trait     |
                  +---------------+
                     /         \
                    /           \
                   v             v
          +---------------+  +---------------+
          |  MemoryState  |  |    RawState   |
          |               |  |               |
          | HashMap based |  | Byte-oriented |
          +---------------+  +---------------+
                    \             /
                     \           /
                      v         v
                     +-----------+
                     |  Overlay  |
                     +-----------+
                          |
                 +--------+--------+
                 |                 |
                 v                 v
              commit            rollback
```

The execution layer is independent from the concrete state representation.

---

## Project Structure

```text
rust-state-engine/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── state.rs
│   ├── memory.rs
│   ├── raw.rs
│   ├── overlay.rs
│   └── evm.rs
├── tests/
│   ├── state_tests.rs
│   └── overlay_tests.rs
└── benches/
    └── state_bench.rs
```

---

## Core State Abstraction

The project defines a generic `State` trait:

```rust
pub trait State {
    type Account<'a>
    where
        Self: 'a;

    type Storage<'a>
    where
        Self: 'a;

    fn account<'a>(
        &'a self,
        address: &Address,
    ) -> Option<Self::Account<'a>>;

    fn storage<'a>(
        &'a self,
        address: &Address,
        slot: &Slot,
    ) -> Option<Self::Storage<'a>>;

    fn balance(
        &self,
        address: &Address,
    ) -> Option<u64>;

    fn set_balance(
        &mut self,
        address: &Address,
        balance: u64,
    );

    fn set_storage(
        &mut self,
        address: &Address,
        slot: &Slot,
        value: &[u8],
    );
}
```

The trait defines the interface without specifying how the state is stored internally.

This allows multiple implementations to expose the same state API while using completely different internal representations.

---

## Generic Associated Types

The state abstraction uses Rust's **Generic Associated Types (GATs)**:

```rust
type Account<'a>
where
    Self: 'a;
```

This allows an implementation to return a type whose lifetime is tied to the lifetime of the state borrow.

For example, `MemoryState` returns:

```rust
AccountRef<'a>
```

while `RawState` returns:

```rust
RawAccount<'a>
```

The generic execution layer does not need to know which representation is being used.

---

## Memory State

`MemoryState` stores structured Rust objects:

```text
HashMap<Address, Account>
HashMap<(Address, Slot), Word>
```

An account contains:

```rust
pub struct Account {
    pub balance: u64,
    pub nonce: u64,
}
```

Account access returns a borrowed reference:

```rust
pub struct AccountRef<'a> {
    pub account: &'a Account,
}
```

No `Account` is copied when reading the state.

---

## Raw State

`RawState` represents a more low-level storage model:

```text
HashMap<Address, Vec<u8>>
HashMap<(Address, Slot), Vec<u8>>
```

Account data is exposed through a borrowed byte slice:

```rust
pub struct RawAccount<'a> {
    pub bytes: &'a [u8],
}
```

The balance and nonce are decoded directly from the underlying byte buffer.

For example:

```rust
let account = state.account(&address).unwrap();

let balance = account.balance();
let nonce = account.nonce();
```

The account data itself is not copied into a new structure.

This demonstrates a basic **zero-copy read path**.

---

## Generic EVM Layer

The execution layer is generic over the state implementation:

```rust
pub struct Evm<S: State> {
    pub state: S,
}
```

The same EVM abstraction can therefore work with:

```rust
Evm<MemoryState>
```

or:

```rust
Evm<RawState>
```

without changing the execution API.

Example:

```rust
let mut evm = Evm::new(state);

evm.set_balance(&address, 2000);

let balance = evm.balance(&address);
```

The EVM does not need to know how the underlying state is represented.

---

## Transaction Overlay

Blockchain execution commonly needs to isolate transaction changes from the base state.

This project implements a simple state overlay.

```text
                 Base State
                     |
                     v
                +----------+
                | Overlay  |
                +----------+
                     |
              transaction changes
```

Suppose the base state contains:

```text
balance = 1000
```

A transaction can modify the overlay:

```text
Base State
balance = 1000

Overlay
balance = 2000
```

The base state remains unchanged until the overlay is committed.

---

## Commit

Calling:

```rust
overlay.commit();
```

applies the pending changes to the underlying state.

Before commit:

```text
Base:     1000
Overlay:  2000
```

After commit:

```text
Base:     2000
Overlay:  empty
```

---

## Rollback

Calling:

```rust
overlay.rollback();
```

discards all pending changes.

Example:

```text
Base:     1000
Overlay:  2000

rollback()

Base:     1000
Overlay:  empty
```

This provides the basic foundation for transaction execution and failure recovery.

---

## Why an Overlay?

The overlay separates:

```text
Persistent/base state
```

from:

```text
Temporary transaction state
```

This allows execution to perform multiple reads and writes without immediately mutating the underlying backend.

A future implementation can extend this model with:

* Nested transactions
* Journaling
* State snapshots
* Reverts
* Write batching
* Persistent storage
* Concurrent execution

---

## Zero-Copy Design

One of the main goals of the project is to avoid unnecessary allocations on read paths.

For `RawState`:

```text
Vec<u8>
   |
   v
&[u8]
   |
   v
RawAccount<'a>
```

The `RawAccount` structure contains a reference to existing bytes rather than allocating a new account object.

This allows the state layer to expose structured views over raw storage.

---

## Ownership Model

Reads borrow from the state:

```rust
let account = state.account(&address);
```

Writes accept borrowed input:

```rust
state.set_storage(
    &address,
    &slot,
    value,
);
```

The state backend takes ownership of the data it needs to retain.

For example:

```rust
value.to_vec()
```

is used when storing raw storage data because the state must own the value after the caller's borrow ends.

This demonstrates the distinction between:

```text
borrowed data for reads
```

and:

```text
owned data for persistent state
```

---

## Rust Concepts Demonstrated

This project intentionally focuses on advanced Rust concepts relevant to systems and blockchain infrastructure.

### Traits

The `State` trait defines a common abstraction for different backends.

### Generic Types

The EVM is generic over its state:

```rust
Evm<S: State>
```

### Associated Types

Each state backend chooses its own account and storage representation.

### GATs

Borrowed associated types are expressed with:

```rust
type Account<'a>
where
    Self: 'a;
```

### Lifetimes

Returned views are tied to the lifetime of the state borrow.

### Borrowing

State reads return references rather than copying stored data.

### Ownership

State writes convert borrowed input into owned storage when necessary.

### HashMap

The memory backend uses hash maps for account and storage lookup.

### Zero-Copy

Raw account data can be interpreted directly from existing byte buffers.

### Transactional State

The overlay provides temporary changes with commit and rollback.

---

## Running the Project

Check the project:

```bash
cargo check
```

Run the tests:

```bash
cargo test
```

Run formatting:

```bash
cargo fmt
```

Run Clippy:

```bash
cargo clippy
```

Run the example:

```bash
cargo run
```

---

## Example Output

Running the example produces:

```text
Base balance: 1000
Overlay balance: 2000
Base balance before commit: 1000
Base balance after commit: 2000
```

This demonstrates that the overlay isolates transaction changes until `commit()` is called.

---

## Testing

The project contains integration tests covering:

* Memory state access
* Raw state access
* Generic EVM execution
* Missing accounts
* Missing storage
* Overlay reads
* Overlay writes
* Commit
* Rollback
* Storage isolation

Run:

```bash
cargo test
```

---

## Design Goals

The project is intentionally small, but the architecture is designed around concepts found in larger blockchain infrastructure systems.

The main goals are:

1. Keep execution independent from storage.
2. Support multiple state representations.
3. Minimize allocations on read paths.
4. Use Rust's type system to express borrowing relationships.
5. Isolate transaction changes from base state.
6. Provide a foundation for persistent storage backends.

---

## Future Work

The project will progressively evolve toward a more realistic blockchain state engine.

### Storage

* Persistent backend
* RocksDB backend
* Key-value abstraction
* Write-ahead logging (WAL)

### LSM Storage

* MemTable
* SSTables
* Compaction
* Bloom filters
* Immutable tables

### State Management

* State snapshots
* Nested overlays
* Transaction journals
* Revert support
* Batch writes

### Blockchain Structures

* Merkleized state
* Patricia/Merkle Patricia Trie concepts
* State roots
* Proof generation
* Proof verification

### Performance

* Criterion benchmarks
* Allocation measurements
* Read/write benchmarks
* Cache-aware data structures
* Concurrent state access

---

## Learning Focus

This project is part of a broader focus on **Rust systems programming and blockchain infrastructure**.

The learning path is centered around:

```text
Rust
  |
  +-- Ownership / Borrowing
  |
  +-- Lifetimes
  |
  +-- Traits
  |
  +-- GATs
  |
  +-- Zero-Copy
  |
  +-- Data Structures
  |
  +-- Storage Engines
  |
  +-- State Management
  |
  +-- Blockchain Infrastructure
```

The objective is to understand not only how to write Rust code, but how to design high-performance, modular systems where ownership, borrowing, storage representation, and execution architecture work together.

---

## License

MIT
