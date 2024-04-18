use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(PoeModule::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::ClaimCreated { sender: 42, claim: 1 }.into());
	});
}