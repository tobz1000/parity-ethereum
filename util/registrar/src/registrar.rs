// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

use call_contract::CallContract;
use ethabi::Address;
use keccak_hash::keccak;
use types::ids::BlockId;

use_contract!(registrar, "res/registrar.json");

// Maps a domain name to an Ethereum address
const DNS_A_RECORD: &'static str = "A";

/// Registrar contract interface
/// Should execute transaction using current blockchain state.
pub trait RegistrarClient: CallContract {
	/// Get registrar address
	fn registrar_address(&self) -> Result<Address, String>;

	fn get_registry_address(&self, name: &str, block: BlockId) -> Result<Option<Address>, String> {
		use ethabi::FunctionOutputDecoder;

		let registrar_address = self.registrar_address()?;
		let hashed_name = keccak(name.as_bytes());
		let (encoded_input, decoder) = registrar::functions::get_address::call(
			hashed_name,
			DNS_A_RECORD
		);
		let encoded_address = self.call_contract(block, registrar_address, encoded_input)?;
		let address = decoder.decode(&encoded_address).map_err(|e| e.to_string())?;

		// TODO - should zero-address be None (w/ return type Result<Option<Bytes>, String>)?
		// Relevant to service_transaction_check::refresh_cache and UrlHintContract::Resolve

		if !address.is_zero() {
			Ok(Some(address))
		} else {
			Ok(None)
		}
	}
}
