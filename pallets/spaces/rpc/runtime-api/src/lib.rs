#![cfg_attr(not(feature = "std"), no_std)]
// #![allow(clippy::too_many_arguments)]

use codec::Codec;
use sp_std::vec::Vec;

use pallet_spaces::rpc::SpaceSerializable;
use pallet_utils::SpaceId;

sp_api::decl_runtime_apis! {
    pub trait SpacesApi<AccountId, BlockNumber> where
        AccountId: Codec,
        BlockNumber: Codec
    {
        fn get_spaces(offset: u64, limit: u64) -> Vec<SpaceSerializable<AccountId, BlockNumber>>;

        fn get_spaces_by_ids(space_ids: Vec<SpaceId>) -> Vec<SpaceSerializable<AccountId, BlockNumber>>;

        fn get_public_spaces(offset: u64, limit: u64) -> Vec<SpaceSerializable<AccountId, BlockNumber>>;

        fn get_unlisted_spaces(offset: u64, limit: u64) -> Vec<SpaceSerializable<AccountId, BlockNumber>>;
    }
}