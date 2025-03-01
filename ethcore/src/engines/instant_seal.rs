// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::BTreeMap;
use util::{Address, HashMap};
use builtin::Builtin;
use engines::{Engine, Seal};
use env_info::EnvInfo;
use spec::CommonParams;
use evm::Schedule;
use block::ExecutedBlock;

/// An engine which does not provide any consensus mechanism, just seals blocks internally.
pub struct InstantSeal {
	params: CommonParams,
	registrar: Address,
	builtins: BTreeMap<Address, Builtin>,
}

impl InstantSeal {
	/// Returns new instance of InstantSeal with default VM Factory
	pub fn new(params: CommonParams, registrar: Address, builtins: BTreeMap<Address, Builtin>) -> Self {
		InstantSeal {
			params: params,
			registrar: registrar,
			builtins: builtins,
		}
	}
}

impl Engine for InstantSeal {
	fn name(&self) -> &str {
		"InstantSeal"
	}

	fn params(&self) -> &CommonParams {
		&self.params
	}

	fn additional_params(&self) -> HashMap<String, String> {
		hash_map!["registrar".to_owned() => self.registrar.hex()]
	}

	fn builtins(&self) -> &BTreeMap<Address, Builtin> {
		&self.builtins
	}

	fn schedule(&self, _env_info: &EnvInfo) -> Schedule {
		Schedule::new_post_eip150(usize::max_value(), true, true, true)
	}

	fn seals_internally(&self) -> Option<bool> { Some(true) }

	fn generate_seal(&self, _block: &ExecutedBlock) -> Seal {
		Seal::Regular(Vec::new())
	}
}

#[cfg(test)]
mod tests {
	use util::*;
	use tests::helpers::*;
	use spec::Spec;
	use header::Header;
	use block::*;
	use engines::Seal;

	#[test]
	fn instant_can_seal() {
		let spec = Spec::new_instant();
		let engine = &*spec.engine;
		let mut db_result = get_temp_state_db();
		let db = spec.ensure_db_good(db_result.take(), &Default::default()).unwrap();
		let genesis_header = spec.genesis_header();
		let last_hashes = Arc::new(vec![genesis_header.hash()]);
		let b = OpenBlock::new(engine, Default::default(), false, db, &genesis_header, last_hashes, Address::default(), (3141562.into(), 31415620.into()), vec![]).unwrap();
		let b = b.close_and_lock();
		if let Seal::Regular(seal) = engine.generate_seal(b.block()) {
			assert!(b.try_seal(engine, seal).is_ok());
		}
	}

	#[test]
	fn instant_cant_verify() {
		let engine = Spec::new_instant().engine;
		let mut header: Header = Header::default();

		assert!(engine.verify_block_basic(&header, None).is_ok());

		header.set_seal(vec![::rlp::encode(&H520::default()).to_vec()]);

		assert!(engine.verify_block_unordered(&header, None).is_ok());
	}
}
