pub use crate::pallet::*;
use frame_support::{
    pallet_prelude::*,
    storage::StoragePrefixedMap,
    traits::GetStorageVersion,
    weights::Weight,
};

use frame_support::{migration::storage_key_iter, Blake2_128Concat};

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct KittyV1 {
    pub dna: [u8; 16],
    pub name: [u8; 4],
}

pub fn on_runtime_upgrade<T: Config>() -> Weight {
    let on_chain_version = Pallet::<T>::on_chain_storage_version();
    let current_version = Pallet::<T>::current_storage_version();

    if on_chain_version != 1 {
        return Weight::zero();
    }

    if current_version != 2 {
        return Weight::zero();
    }

    let module = Kitties::<T>::module_prefix();
    let item = Kitties::<T>::storage_prefix();

    for (index, kitty) in storage_key_iter::<KittyIndex, KittyV1, Blake2_128Concat>(module, item).drain() {
        let mut name = [0u8; 8];
        for i in 0..name.len() {
            name[i] = kitty.name[i%4];
        }
        let new_kitty = Kitty {
            dna: kitty.dna,
            name,
        };
        Kitties::<T>::insert(index, &new_kitty);
    }
    Weight::zero()
}