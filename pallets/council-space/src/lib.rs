#![cfg_attr(not(feature = "std"), no_std)]

// use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, /* ensure,*/
    // dispatch::{DispatchError, DispatchResult},
    traits::Get,
};
use sp_runtime::{/*RuntimeDebug, */KeyTypeId};
use sp_std::prelude::*;
// use sp_runtime::transaction_validity::{
//     TransactionValidity, TransactionLongevity, ValidTransaction, InvalidTransaction,
// };
use frame_system::{
    self as system/*, ensure_signed, offchain,*/
};

use pallet_utils::{SpaceId};
// use pallet_spaces::Space;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"pcso");

pub mod crypto {
    pub use super::KEY_TYPE;
    use sp_runtime::app_crypto::{app_crypto, sr25519};
    app_crypto!(sr25519, KEY_TYPE);
}

pub trait Trait<I: Instance = DefaultInstance>: system::Trait
{
    /// The overarching event type.
    type Event: From<Event<Self, I>> + Into<<Self as frame_system::Trait>::Event>;

    type NetworkId: Get<Vec<u8>>;
}

decl_error! {
    pub enum Error for Module<T: Trait<I>, I: Instance> {}
}

// This pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait<I>, I: Instance=DefaultInstance> as SpacesModule {

        pub Owner get(fn owner): Option<T::AccountId>;

        pub CouncilSpaceId get(fn council_space_id): Option<SpaceId>;

        pub NetworkId get(fn network_id): Vec<u8>;
    }
}

decl_event!(
    pub enum Event<T, I=DefaultInstance> where
        <T as system::Trait>::AccountId,
    {
        TemplateEvent(AccountId),
    }
);

// The pallet's dispatchable functions.
decl_module! {
    pub struct Module<T: Trait<I>, I: Instance=DefaultInstance> for enum Call where origin: T::Origin {

        // Initializing errors
        type Error = Error<T, I>;

        // Initializing events
        fn deposit_event() = default;

        const NetworkId: Vec<u8> = T::NetworkId::get();

        // #[weight = 100_000 + T::DbWeight::get().reads_writes(4, 4)]
    }
}

impl<T: Trait> Module<T> {}
