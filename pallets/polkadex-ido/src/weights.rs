// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for polkadex_ido
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-05-07, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/polkadex-node
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=polkadex-ido
// --extrinsic=*
// --steps=50
// --repeat=20
// --heap-pages=4096
// --template=./frame-weight-template.hbs
// --output=./pallets/Polkadex_Ido/src/weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for polkadex_ido.
pub trait WeightInfo {
    fn register_investor() -> Weight;
    fn register_round() -> Weight;
    fn whitelist_investor() -> Weight;
    fn claim_tokens() -> Weight;
    fn show_interest_in_round() -> Weight;
}

/// Weights for polkadex_ido using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn register_investor() -> Weight {
        (25_000_000_u64)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    fn register_round() -> Weight {
        (24_000_000_u64)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    fn whitelist_investor() -> Weight {
        (29_000_000_u64)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    fn claim_tokens() -> Weight {
        (33_000_000_u64)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    fn show_interest_in_round() -> Weight {
        (26_000_000_u64)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn register_investor() -> Weight {
        (25_000_000_u64)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    fn register_round() -> Weight {
        (24_000_000_u64)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    fn whitelist_investor() -> Weight {
        (29_000_000_u64)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    fn claim_tokens() -> Weight {
        (33_000_000_u64)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    fn show_interest_in_round() -> Weight {
        (26_000_000_u64)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}