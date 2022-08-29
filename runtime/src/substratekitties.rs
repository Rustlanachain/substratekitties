use support::{decl_storage, decl_module, StorageValue,dispatch::Result,StorageMap};
use system::ensure_signed;

pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // Declare storage and getter functions here
        Value: map T::AccountId => u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here
        fn set_value(origin, value: u64) -> Result {
            let sender = ensure_signed(origin)?;

            <Value<T>>::insert(sender, value);

            Ok(())
        }
    }
}
