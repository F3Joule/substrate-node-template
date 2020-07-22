use crate::{
    mock::*, Error,
    vec_remove_on, log_2,
    Content,
};

use sp_std::iter::FromIterator;
use frame_support::{assert_ok, assert_noop};

#[test]
fn log_2_should_work() {
    ExtBuilder::build().execute_with(|| {
        // None should be returned if zero (0) is provided
        assert!(log_2(0).is_none());

        // Log2 of 1 should be zero (0)
        assert_eq!(log_2(1), Some(0));

        // Log2 of 2 should be 1
        assert_eq!(log_2(2), Some(1));

        // Log2 of 128 should be 7
        assert_eq!(log_2(128), Some(7));

        // Log2 of 512 should be 9
        assert_eq!(log_2(512), Some(9));

        // Log2 of u32::MAX (4294967295) should be 31
        assert_eq!(log_2(u32::MAX), Some(31));
    });
}

#[test]
fn vec_remove_on_should_work_with_zero_elements() {
    ExtBuilder::build().execute_with(|| {
        let element: u16 = 2;
        let vector: &mut Vec<u16> = &mut vec![];

        vec_remove_on(vector, element);
        assert!(vector.is_empty());
    });
}

#[test]
fn vec_remove_on_should_work_with_last_element() {
    ExtBuilder::build().execute_with(|| {
        let element: u16 = 2;
        let vector: &mut Vec<u16> = &mut vec![6, 2];

        vector.remove(0);
        assert_eq!(vector, &mut vec![2]);

        vec_remove_on(vector, element);
        assert!(vector.is_empty());
    });
}

#[test]
fn vec_remove_on_should_work_with_two_elements() {
    ExtBuilder::build().execute_with(|| {
        let element: u16 = 2;
        let vector: &mut Vec<u16> = &mut vec![6, 2, 7];

        vector.remove(0);
        assert_eq!(vector, &mut vec![2, 7]);

        vec_remove_on(vector, element);
        assert_eq!(vector, &mut vec![7]);
    });
}

#[test]
fn convert_users_vec_to_btree_set_should_work() {
    ExtBuilder::build().execute_with(|| {
        // Empty vector should produce empty set
        assert_eq!(
            _convert_users_vec_to_btree_set(vec![]).ok().unwrap(),
            UsersSet::new()
        );

        assert_eq!(
            _convert_users_vec_to_btree_set(vec![USER1]).ok().unwrap(),
            UsersSet::from_iter(vec![USER1].into_iter())
        );

        // Duplicates should produce 1 unique element
        assert_eq!(
            _convert_users_vec_to_btree_set(vec![USER1, USER1, USER3]).ok().unwrap(),
            UsersSet::from_iter(vec![USER1, USER3].into_iter())
        );

        // Randomly filled vec should produce sorted set
        assert_eq!(
            _convert_users_vec_to_btree_set(vec![USER3, USER1, USER3, USER2, USER1]).ok().unwrap(),
            UsersSet::from_iter(vec![USER1, USER2, USER3].into_iter())
        );
    });
}

use cid::{Cid, Codec, Version};
use multihash::Sha2_256;

#[test]
fn is_valid_content_should_work_with_ipfs_cid_v0() {
    ExtBuilder::build().execute_with(|| {
        let valid_cid_v0 = Cid::new(
            Version::V0,
            Codec::DagProtobuf,
            Sha2_256::digest(b"valid cid of version 0")
        ).unwrap().to_bytes();

        assert_ok!(_is_valid_content(Content::IPFS(valid_cid_v0)));
    });
}

#[test]
fn is_valid_content_should_work_with_ipfs_cid_v1() {
    ExtBuilder::build().execute_with(|| {
        let valid_cid_v1 = Cid::new(
            Version::V1,
            Codec::DagProtobuf,
            Sha2_256::digest(b"valid cid of version 1")
        ).unwrap().to_bytes();

        assert_ok!(_is_valid_content(Content::IPFS(valid_cid_v1)));
    });
}

#[test]
fn is_valid_content_should_fail_with_invalid_ipfs_cid() {
    ExtBuilder::build().execute_with(|| {
        let invalid_cid = b"QmYHzA8euDgUpNy3fht6jCgF35YTutYkyGGyr8f".to_vec();

        assert_noop!(_is_valid_content(Content::IPFS(invalid_cid)), Error::<Test>::InvalidIpfsCid);
    });
}
