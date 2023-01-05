// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_message_queue
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm2`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_message_queue
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/message-queue/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_message_queue.
pub trait WeightInfo {
	fn ready_ring_knit() -> Weight;
	fn ready_ring_unknit() -> Weight;
	fn service_queue_base() -> Weight;
	fn service_page_base_completion() -> Weight;
	fn service_page_base_no_completion() -> Weight;
	fn service_page_item() -> Weight;
	fn bump_service_head() -> Weight;
	fn reap_page() -> Weight;
	fn execute_overweight_page_removed() -> Weight;
	fn execute_overweight_page_updated() -> Weight;
}

/// Weights for pallet_message_queue using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn ready_ring_knit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `303`
		//  Estimated: `5554`
		// Minimum execution time: 10_296 nanoseconds.
		Weight::from_parts(10_579_000, 5554)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	fn ready_ring_unknit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `303`
		//  Estimated: `5554`
		// Minimum execution time: 10_261 nanoseconds.
		Weight::from_parts(10_542_000, 5554)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn service_queue_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `2527`
		// Minimum execution time: 4_185 nanoseconds.
		Weight::from_parts(4_312_000, 2527)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn service_page_base_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `68060`
		// Minimum execution time: 5_541 nanoseconds.
		Weight::from_parts(5_689_000, 68060)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn service_page_base_no_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `68060`
		// Minimum execution time: 5_657 nanoseconds.
		Weight::from_parts(5_868_000, 68060)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn service_page_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 73_640 nanoseconds.
		Weight::from_ref_time(73_980_000)
	}
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:0)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn bump_service_head() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178`
		//  Estimated: `3027`
		// Minimum execution time: 6_346 nanoseconds.
		Weight::from_parts(6_582_000, 3027)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn reap_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65744`
		//  Estimated: `70587`
		// Minimum execution time: 51_597 nanoseconds.
		Weight::from_parts(52_178_000, 70587)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn execute_overweight_page_removed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65744`
		//  Estimated: `70587`
		// Minimum execution time: 66_000 nanoseconds.
		Weight::from_parts(66_463_000, 70587)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn execute_overweight_page_updated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65744`
		//  Estimated: `70587`
		// Minimum execution time: 78_320 nanoseconds.
		Weight::from_parts(78_982_000, 70587)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn ready_ring_knit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `303`
		//  Estimated: `5554`
		// Minimum execution time: 10_296 nanoseconds.
		Weight::from_parts(10_579_000, 5554)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	fn ready_ring_unknit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `303`
		//  Estimated: `5554`
		// Minimum execution time: 10_261 nanoseconds.
		Weight::from_parts(10_542_000, 5554)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn service_queue_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `2527`
		// Minimum execution time: 4_185 nanoseconds.
		Weight::from_parts(4_312_000, 2527)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn service_page_base_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `68060`
		// Minimum execution time: 5_541 nanoseconds.
		Weight::from_parts(5_689_000, 68060)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn service_page_base_no_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `68060`
		// Minimum execution time: 5_657 nanoseconds.
		Weight::from_parts(5_868_000, 68060)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn service_page_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 73_640 nanoseconds.
		Weight::from_ref_time(73_980_000)
	}
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:0)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn bump_service_head() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178`
		//  Estimated: `3027`
		// Minimum execution time: 6_346 nanoseconds.
		Weight::from_parts(6_582_000, 3027)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn reap_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65744`
		//  Estimated: `70587`
		// Minimum execution time: 51_597 nanoseconds.
		Weight::from_parts(52_178_000, 70587)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn execute_overweight_page_removed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65744`
		//  Estimated: `70587`
		// Minimum execution time: 66_000 nanoseconds.
		Weight::from_parts(66_463_000, 70587)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn execute_overweight_page_updated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65744`
		//  Estimated: `70587`
		// Minimum execution time: 78_320 nanoseconds.
		Weight::from_parts(78_982_000, 70587)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
