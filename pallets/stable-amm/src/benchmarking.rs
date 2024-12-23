//! Benchmarking setup for pallet-stable-amm
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as StableAMM;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	// #[benchmark]
	// fn do_something() {
	// 	let value = 100u32.into();
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	#[extrinsic_call]
	// 	do_something(RawOrigin::Signed(caller), value);

	// 	assert_eq!(Something::<T>::get(), Some(value));
	// }

	// #[benchmark]
	// fn cause_error() {
	// 	Something::<T>::put(100u32);
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	#[extrinsic_call]
	// 	cause_error(RawOrigin::Signed(caller));

	// 	assert_eq!(Something::<T>::get(), Some(101u32));
	// }

	// impl_benchmark_test_suite!(StableAMM, crate::mock::new_test_ext(), crate::mock::Test);
}
