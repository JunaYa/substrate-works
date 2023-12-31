use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_test() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0; 1]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
		// 断言链上状态
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn create_claim_fail_when_claim_already_test() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0; 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		// 已存在，
		// 不会对链上状态造成影响
		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0; 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
		// 断言链上状态
		assert_eq!(Proofs::<Test>::get(&claim), None);
	});
}

#[test]
fn revoke_claim_fail_when_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0; 1]).unwrap();
		// 不存在，
		// 不会对链上状态造成影响
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	});
}

// #[test]
// fn transfer_claim_works() {
// 	new_test_ext().execute_with(|| {
// 		let claim = BoundedVec::try_from(vec![0; 1]).unwrap();
// 		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
// 		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2));
// 		// 断言链上状态
// 		assert_eq!(
// 			Proofs::<Test>::get(&claim),
// 			Some((2, frame_system::Pallet::<Test>::block_number()))
// 		);
// 	});
// }
