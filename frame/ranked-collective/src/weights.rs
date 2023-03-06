// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_ranked_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-uscvwkiz-project-145-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_ranked_collective
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/ranked-collective/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_ranked_collective.
pub trait WeightInfo {
	fn add_member() -> Weight;
	fn remove_member(r: u32, ) -> Weight;
	fn promote_member(r: u32, ) -> Weight;
	fn demote_member(r: u32, ) -> Weight;
	fn vote() -> Weight;
	fn cleanup_poll(n: u32, ) -> Weight;
}

/// Weights for pallet_ranked_collective using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IndexToId (r:0 w:1)
	/// Proof: RankedCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:0 w:1)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn add_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `6986`
		// Minimum execution time: 18_976 nanoseconds.
		Weight::from_ref_time(19_426_000)
			.saturating_add(Weight::from_proof_size(6986))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:11 w:11)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:11 w:11)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: RankedCollective IndexToId (r:11 w:11)
	/// Proof: RankedCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 10]`.
	fn remove_member(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `583 + r * (281 ±0)`
		//  Estimated: `14024 + r * (7547 ±0)`
		// Minimum execution time: 32_161 nanoseconds.
		Weight::from_ref_time(36_262_498)
			.saturating_add(Weight::from_proof_size(14024))
			// Standard Error: 25_088
			.saturating_add(Weight::from_ref_time(15_339_326).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_proof_size(7547).saturating_mul(r.into()))
	}
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IndexToId (r:0 w:1)
	/// Proof: RankedCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:0 w:1)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 10]`.
	fn promote_member(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `281 + r * (17 ±0)`
		//  Estimated: `6986`
		// Minimum execution time: 21_963 nanoseconds.
		Weight::from_ref_time(23_481_059)
			.saturating_add(Weight::from_proof_size(6986))
			// Standard Error: 6_703
			.saturating_add(Weight::from_ref_time(397_433).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:1 w:1)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: RankedCollective IndexToId (r:1 w:1)
	/// Proof: RankedCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 10]`.
	fn demote_member(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `599 + r * (72 ±0)`
		//  Estimated: `14024`
		// Minimum execution time: 32_423 nanoseconds.
		Weight::from_ref_time(35_459_465)
			.saturating_add(Weight::from_proof_size(14024))
			// Standard Error: 28_877
			.saturating_add(Weight::from_ref_time(889_204).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: RankedPolls ReferendumInfoFor (r:1 w:1)
	/// Proof: RankedPolls ReferendumInfoFor (max_values: None, max_size: Some(330), added: 2805, mode: MaxEncodedLen)
	/// Storage: RankedCollective Voting (r:1 w:1)
	/// Proof: RankedCollective Voting (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(107022), added: 109497, mode: MaxEncodedLen)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `626`
		//  Estimated: `230816`
		// Minimum execution time: 53_929 nanoseconds.
		Weight::from_ref_time(56_428_000)
			.saturating_add(Weight::from_proof_size(230816))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: RankedPolls ReferendumInfoFor (r:1 w:0)
	/// Proof: RankedPolls ReferendumInfoFor (max_values: None, max_size: Some(330), added: 2805, mode: MaxEncodedLen)
	/// Storage: RankedCollective VotingCleanup (r:1 w:0)
	/// Proof: RankedCollective VotingCleanup (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: RankedCollective Voting (r:100 w:100)
	/// Proof: RankedCollective Voting (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	fn cleanup_poll(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `461 + n * (50 ±0)`
		//  Estimated: `8364 + n * (2540 ±0)`
		// Minimum execution time: 16_208 nanoseconds.
		Weight::from_ref_time(20_668_502)
			.saturating_add(Weight::from_proof_size(8364))
			// Standard Error: 2_956
			.saturating_add(Weight::from_ref_time(1_292_296).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(2540).saturating_mul(n.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IndexToId (r:0 w:1)
	/// Proof: RankedCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:0 w:1)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn add_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `6986`
		// Minimum execution time: 18_976 nanoseconds.
		Weight::from_ref_time(19_426_000)
			.saturating_add(Weight::from_proof_size(6986))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:11 w:11)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:11 w:11)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: RankedCollective IndexToId (r:11 w:11)
	/// Proof: RankedCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 10]`.
	fn remove_member(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `583 + r * (281 ±0)`
		//  Estimated: `14024 + r * (7547 ±0)`
		// Minimum execution time: 32_161 nanoseconds.
		Weight::from_ref_time(36_262_498)
			.saturating_add(Weight::from_proof_size(14024))
			// Standard Error: 25_088
			.saturating_add(Weight::from_ref_time(15_339_326).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_proof_size(7547).saturating_mul(r.into()))
	}
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IndexToId (r:0 w:1)
	/// Proof: RankedCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:0 w:1)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 10]`.
	fn promote_member(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `281 + r * (17 ±0)`
		//  Estimated: `6986`
		// Minimum execution time: 21_963 nanoseconds.
		Weight::from_ref_time(23_481_059)
			.saturating_add(Weight::from_proof_size(6986))
			// Standard Error: 6_703
			.saturating_add(Weight::from_ref_time(397_433).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:1 w:1)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: RankedCollective IndexToId (r:1 w:1)
	/// Proof: RankedCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 10]`.
	fn demote_member(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `599 + r * (72 ±0)`
		//  Estimated: `14024`
		// Minimum execution time: 32_423 nanoseconds.
		Weight::from_ref_time(35_459_465)
			.saturating_add(Weight::from_proof_size(14024))
			// Standard Error: 28_877
			.saturating_add(Weight::from_ref_time(889_204).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: RankedPolls ReferendumInfoFor (r:1 w:1)
	/// Proof: RankedPolls ReferendumInfoFor (max_values: None, max_size: Some(330), added: 2805, mode: MaxEncodedLen)
	/// Storage: RankedCollective Voting (r:1 w:1)
	/// Proof: RankedCollective Voting (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(107022), added: 109497, mode: MaxEncodedLen)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `626`
		//  Estimated: `230816`
		// Minimum execution time: 53_929 nanoseconds.
		Weight::from_ref_time(56_428_000)
			.saturating_add(Weight::from_proof_size(230816))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: RankedPolls ReferendumInfoFor (r:1 w:0)
	/// Proof: RankedPolls ReferendumInfoFor (max_values: None, max_size: Some(330), added: 2805, mode: MaxEncodedLen)
	/// Storage: RankedCollective VotingCleanup (r:1 w:0)
	/// Proof: RankedCollective VotingCleanup (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: RankedCollective Voting (r:100 w:100)
	/// Proof: RankedCollective Voting (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	fn cleanup_poll(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `461 + n * (50 ±0)`
		//  Estimated: `8364 + n * (2540 ±0)`
		// Minimum execution time: 16_208 nanoseconds.
		Weight::from_ref_time(20_668_502)
			.saturating_add(Weight::from_proof_size(8364))
			// Standard Error: 2_956
			.saturating_add(Weight::from_ref_time(1_292_296).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(2540).saturating_mul(n.into()))
	}
}
