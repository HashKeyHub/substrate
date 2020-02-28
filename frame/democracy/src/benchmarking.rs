// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Democracy pallet benchmarking.

use super::*;

use frame_system::{RawOrigin, self};
use sp_io::hashing::blake2_256;
use frame_benchmarking::{benchmarks, account};
use sp_core::hash::H256;
use sp_runtime::traits::{Bounded, Dispatchable, StaticLookup};
use frame_support::traits::{Currency, Get};

use crate::Module as Democracy;

const MAX_PROPOSALS: u32 = 100;

benchmarks! {
	_ {
		let p in 0 .. MAX_PROPOSALS => ();
	}

	propose {
		let p in ...;

		let caller: T::AccountId = account("caller", 0, 0);
		let caller_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(caller.clone());
		let mut proposal_hash: T::Hash = Default::default();
		#[cfg(feature = "test")]
		{
			proposal_hash = H256::random();
		}

		T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
		let value = 1;

	}: _(RawOrigin::Signed(caller), proposal_hash, value.into())
}