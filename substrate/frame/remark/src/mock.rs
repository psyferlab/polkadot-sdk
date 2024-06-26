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

//! Test environment for remarks pallet.

use crate as pallet_remark;
use frame_support::derive_impl;
use sp_runtime::BuildStorage;

pub type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Remark: pallet_remark,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

impl pallet_remark::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = RuntimeGenesisConfig { system: Default::default() }.build_storage().unwrap();
	t.into()
}
