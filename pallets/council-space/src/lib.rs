#![cfg_attr(not(feature = "std"), no_std)]

// use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,  ensure,
    dispatch::{/*DispatchError, */DispatchResult},
    traits::Get,
};
/*use sp_runtime::{
    // RuntimeDebug,
    // offchain::{http, Duration, storage::StorageValueRef},
    /*traits::Zero,
    transaction_validity::{
        InvalidTransaction, ValidTransaction, TransactionValidity, TransactionSource,
        TransactionPriority,
    },*/
};*/
use sp_std::prelude::*;
use sp_core::crypto::KeyTypeId;
use frame_system::{
    self as system, ensure_signed,
    offchain::{
        /*AppCrypto, */CreateSignedTransaction/*, SendUnsignedTransaction, SendSignedTransaction,
        SignedPayload, SigningTypes, Signer, SubmitTransaction,*/
    },
};

use pallet_utils::{SpaceId};
use pallet_multisig::Module as MultisigModule;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"pcso");

pub mod crypto {
    use super::KEY_TYPE;
    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
    };
    use sp_core::sr25519::Signature as Sr25519Signature;
    app_crypto!(sr25519, KEY_TYPE);

    pub struct CouncilSpaceAuthId;
    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for CouncilSpaceAuthId {
        type RuntimeAppPublic = Public;
        type GenericPublic = sp_core::sr25519::Public;
        type GenericSignature = sp_core::sr25519::Signature;
    }
}

pub trait Trait<I: Instance = DefaultInstance>:
    CreateSignedTransaction<Call<Self, I>>
    + pallet_multisig::Trait
{
    // /// The identifier type for an offchain worker.
    // type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

    /// The overarching event type.
    type Event: From<Event<Self, I>> + Into<<Self as frame_system::Trait>::Event>;
    /// The overarching dispatch call type.
    type Call: From<Call<Self, I>>;

    type NetworkRpcAddress: Get<Vec<u8>>;
}

decl_error! {
    pub enum Error for Module<T: Trait<I>, I: Instance> {
        OwnerForThisCouncilNotSet,
        SenderIsNotAnOwner,
    }
}

// This pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait<I>, I: Instance=DefaultInstance> as SpacesModule {
        pub Owner get(fn owner): Option<T::AccountId>;
        pub CouncilSpaceId get(fn council_space_id): Option<SpaceId>;
    }
}

decl_event!(
    pub enum Event<T, I=DefaultInstance> where
        <T as system::Trait>::AccountId,
    {
        NewCouncilSpaceSet(AccountId, Option<SpaceId>),
    }
);

// The pallet's dispatchable functions.
decl_module! {
    pub struct Module<T: Trait<I>, I: Instance=DefaultInstance> for enum Call where origin: T::Origin {

        // Initializing errors
        type Error = Error<T, I>;

        // Initializing events
        fn deposit_event() = default;

        const NetworkRpcAddress: Vec<u8> = T::NetworkRpcAddress::get();

        fn offchain_worker(block: T::BlockNumber) {
            if block % T::BlockNumber::from(6) == T::BlockNumber::from(0) {
                let (mut members, threshold) = Self::fetch_council();

                members.sort();

                let multi_account_id = MultisigModule::<T>::multi_account_id(&members, threshold);

                if let Some(owner) = Self::owner() {
                    if owner != multi_account_id {
                        Owner::<T, I>::set(Some(multi_account_id));
                    }
                } else {
                    Owner::<T, I>::set(Some(multi_account_id));
                }
            }
        }

        #[weight = 10_000 + T::DbWeight::get().reads_writes(0, 0)]
        pub fn set_space_id(origin, new_space_id: Option<SpaceId>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let owner = Self::owner().ok_or(Error::<T, I>::OwnerForThisCouncilNotSet)?;
            ensure!(sender == owner, Error::<T, I>::SenderIsNotAnOwner);

            CouncilSpaceId::<I>::set(new_space_id);

            Self::deposit_event(RawEvent::NewCouncilSpaceSet(owner, new_space_id));
            Ok(())
        }
    }
}

impl<T: Trait<I>, I: Instance> Module<T,I> {
    fn fetch_council() -> (Vec<T::AccountId>, u16) {
        unimplemented!()
    }
}
