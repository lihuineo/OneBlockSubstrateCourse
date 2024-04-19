use crate::{mock::*, Event};
use frame_support::{assert_ok, traits::ConstU32};
use sp_runtime::BoundedVec;

#[test]
fn create_claim_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let data = BoundedVec::<u8, ConstU32<512>>::try_from(vec![1; 10].clone()).unwrap();
		let account_id = 18_446_744_073_709_551_615u64;

		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(account_id), data.clone()));
		assert_eq!(PoeModule::proofs(&data).unwrap().0, account_id);
		System::assert_last_event(Event::ClaimCreated(account_id, data.clone()).into());
	});
}
