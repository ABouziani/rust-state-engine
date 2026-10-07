pub type Address = [u8; 20];
pub type Slot = [u8; 32];
pub type Word = [u8; 32];

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

    fn balance(&self, address: &Address) -> Option<u64>;

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