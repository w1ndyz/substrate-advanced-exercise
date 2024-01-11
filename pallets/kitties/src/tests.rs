use crate::{Error, mock::{*}};
use frame_support::{assert_ok, assert_noop};
#[test]
fn it_works_for_create() {
  new_test_ext().execute_with(|| {
    let kitty_id = 0;
    let account_id = 1;

    assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
    assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));


    assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
    assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
    assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
    assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

    crate::NextKittyId::<Test>::set(crate::KittyIndex::max_value());
    assert_noop!(
      KittiesModule::create(RuntimeOrigin::signed(account_id)),
      Error::<Test>::InvalidKittyId
    );

    let kitty = KittiesModule::kitties(kitty_id).unwrap();
    System::assert_has_event(RuntimeEvent::KittiesModule(crate::Event::KittyCreated {
      who: account_id,
      kitty_id: kitty_id,
      kitty: kitty,
    }));
  });
}

#[test]
fn it_works_for_breed() {
  new_test_ext().execute_with(|| {
    let kitty_id = 0;
    let account_id = 1;

    assert_noop!(
      KittiesModule::bred(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
      Error::<Test>::SameKittyId
    );

    assert_noop!(
      KittiesModule::bred(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1),
      Error::<Test>::InvalidKittyId
    );

    assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
    assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

    assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

    assert_ok!(KittiesModule::bred(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1));

    let breed_kitty_id = 2;
    assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
    assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
    assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
    assert_eq!(
      KittiesModule::kitty_parents(breed_kitty_id),
      Some((kitty_id, kitty_id + 1))
    );

    let kitty = KittiesModule::kitties(breed_kitty_id).unwrap();
    System::assert_has_event(RuntimeEvent::KittiesModule(crate::Event::KittyBred {
      who: account_id,
      kitty_id: breed_kitty_id,
      kitty: kitty,
    }));
  });
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
      KittiesModule::transfer(RuntimeOrigin::signed(recipient), recipient, kitty_id),
      Error::<Test>::NotOwner
    );

    assert_ok!(KittiesModule::transfer(
      RuntimeOrigin::signed(account_id),
      recipient,
      kitty_id
    ));

    assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));

    assert_ok!(KittiesModule::transfer(
      RuntimeOrigin::signed(recipient),
      account_id,
      kitty_id
    ));

    assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

    System::assert_has_event(RuntimeEvent::KittiesModule(crate::Event::KittyTransferred {
      who: account_id,
      recipient: recipient,
      kitty_id: kitty_id,
    }));
  });
}

#[test]
fn it_works_on_sale() {
    new_test_ext().execute_with(|| {
      let kitty_id = 0;
      let account_id = 1;

      assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
      assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
      assert_noop!(
        KittiesModule::sale(RuntimeOrigin::signed(account_id + 1), kitty_id),
        Error::<Test>::NotOwner
      );
      assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id));

      System::assert_has_event(RuntimeEvent::KittiesModule(crate::Event::KittyOnSale {
        who: account_id,
        kitty_id: kitty_id,
      }));
    });
}

#[test]
fn it_works_on_bought() {
  new_test_ext().execute_with(|| {
    let kitty_id = 0;
    let account_id = 1;
    let buyer = 2;

    assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
    assert_ok!(KittiesModule::create(RuntimeOrigin::signed(buyer)));
    assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
    assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id));
    assert_noop!(
      KittiesModule::buy(RuntimeOrigin::signed(account_id), kitty_id),
      Error::<Test>::AlreadyOwned
    );
    assert_ok!(KittiesModule::buy(RuntimeOrigin::signed(buyer), kitty_id));

    assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(buyer));

    System::assert_has_event(RuntimeEvent::KittiesModule(crate::Event::KittyBought {
      who: buyer,
      kitty_id: kitty_id,
    }));
  });
}