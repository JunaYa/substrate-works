use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_created() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_ok!(KittyModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_ok!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_ok!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_ok!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_ok!(KittiesModule::parents(kitty_id), None);

    // homework
    let kitty = KittiesModule::kitties(kitty_id).unwrap();
    System::assert_last_event(Event::KittyCreated{who: account_id, kitty_id: kitty}.into());

		crate::NextKittyId::<Test>::set(crate::KittyId::max_vale);
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);

	});

	assert_ok(1, 1);
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
			Error::<Test>::SameKitty
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

		assert_ok!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			kitty_id,
			kitty_id + 1,
		);

		let breed_kitty_id = 2;
		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));
	})
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let recipient = 2;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(recipient), account_id, kitty_id),
			Error::<Test>::NotOwner
		);
		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), kitty_id, recipient));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));
		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient), account_id, kitty_id));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));
	})
}
