//! Benchmarking setup for pallet-poe

use super::*;

#[allow(unused)]
use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;

fn assert_success_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}
fn insert_claim<T: Config>(claim: &Vec<u8>, sender: &T::AccountId) {
    let bounded_claim =
        BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).expect("claim too long");
    Proofs::<T>::insert(
        &bounded_claim,
        (sender.clone(), frame_system::Pallet::<T>::block_number()),
    );
}


benchmarks! {
	create_claim {
		let d in 0..T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();
	} : _(RawOrigin::Signed(caller.clone()), claim.clone())
	verify {
		assert_success_event::<T>(
			Event::ClaimCreated(caller, claim.clone()).into()
		);
	}

	transfer_claim {
		let d in 0..T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		let dest: T::AccountId = account("dest", 0, 0);

		insert_claim::<T>(&claim, &caller);
	} : _(RawOrigin::Signed(caller.clone()), claim.clone(), dest.clone())
	verify {
		assert_success_event::<T>(
			Event::ClaimTransferred(caller.clone(), dest.clone(), claim).into()
		);
	}

	revoke_claim {
		let d in 0..T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();

		insert_claim::<T>(&claim, &caller);
	} : _(RawOrigin::Signed(caller.clone()), claim.clone())
	verify {
		assert_success_event::<T>(
			Event::ClaimRevoked(caller, claim).into()
		);
	}

	impl_benchmark_test_suite!(PoeModule, crate::mock::new_test_ext(), crate::mock::Test);
}