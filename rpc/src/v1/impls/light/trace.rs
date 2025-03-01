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

//! Traces api implementation.

use jsonrpc_core::Error;
use jsonrpc_macros::Trailing;
use v1::traits::Traces;
use v1::helpers::errors;
use v1::types::{TraceFilter, LocalizedTrace, BlockNumber, Index, CallRequest, Bytes, TraceResults, H256};

/// Traces api implementation.
// TODO: all calling APIs should be possible w. proved remote TX execution.
pub struct TracesClient;

impl Traces for TracesClient {
	fn filter(&self, _filter: TraceFilter) -> Result<Vec<LocalizedTrace>, Error> {
		Err(errors::light_unimplemented(None))
	}

	fn block_traces(&self, _block_number: BlockNumber) -> Result<Vec<LocalizedTrace>, Error> {
		Err(errors::light_unimplemented(None))
	}

	fn transaction_traces(&self, _transaction_hash: H256) -> Result<Vec<LocalizedTrace>, Error> {
		Err(errors::light_unimplemented(None))
	}

	fn trace(&self, _transaction_hash: H256, _address: Vec<Index>) -> Result<Option<LocalizedTrace>, Error> {
		Err(errors::light_unimplemented(None))
	}

	fn call(&self, _request: CallRequest, _flags: Vec<String>, _block: Trailing<BlockNumber>) -> Result<Option<TraceResults>, Error> {
		Err(errors::light_unimplemented(None))
	}

	fn raw_transaction(&self, _raw_transaction: Bytes, _flags: Vec<String>, _block: Trailing<BlockNumber>) -> Result<Option<TraceResults>, Error> {
		Err(errors::light_unimplemented(None))
	}

	fn replay_transaction(&self, _transaction_hash: H256, _flags: Vec<String>) -> Result<Option<TraceResults>, Error> {
		Err(errors::light_unimplemented(None))
	}
}
