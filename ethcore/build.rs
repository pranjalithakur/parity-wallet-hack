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

extern crate ethcore_ipc_codegen;

fn main() {
	ethcore_ipc_codegen::derive_binary("src/types/mod.rs.in").unwrap();
	ethcore_ipc_codegen::derive_ipc_cond("src/client/traits.rs", cfg!(feature="ipc")).unwrap();
	ethcore_ipc_codegen::derive_ipc_cond("src/snapshot/snapshot_service_trait.rs", cfg!(feature="ipc")).unwrap();
	ethcore_ipc_codegen::derive_ipc_cond("src/client/chain_notify.rs", cfg!(feature="ipc")).unwrap();
}
