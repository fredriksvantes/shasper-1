// Copyright 2018 Parity Technologies (UK) Ltd.
// This file is part of Substrate Shasper.

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

use primitives::{BitField, H256, Signature};

pub struct Crosslink {
	/// Epoch number
	pub epoch: u64,
	/// Shard data since the previous crosslink
	pub crosslink_data_root: H256,
}

pub struct Attestation {
	/// Attester aggregation bitfield
	pub aggregation_bitfield: BitField,
	/// Attestation data
	pub data: AttestationData,
	/// Custody bitfield
	pub custody_bitfield: BitField,
	/// BLS aggregate signature
	pub aggregate_signature: Signature,
}

pub struct PendingAttestation {
	/// Attester aggregation bitfield
	pub aggregation_bitfield: BitField,
	/// Attestation data
	pub data: AttestationData,
	/// Custody bitfield
	pub custody_bitfield: BitField,
	/// Inclusion slot
	pub inclusion_slot: u64,
}

pub struct AttestationData {
	/// Slot number
	pub slot: u64,
	/// Shard number
	pub shard: u64,
	/// Root of the signed beacon block
	pub beacon_block_root: H256,
	/// Root of the ancestor at the epoch boundary
	pub epoch_boundary_root: H256,
	/// Data from the shard since the last attestation
	pub crosslink_data_root: H256,
	/// Last crosslink
	pub latest_crosslink: Crosslink,
	/// Last justified epoch in the beacon state
	pub justified_epoch: u64,
	/// Hash of the last justified beacon block
	pub justified_block_root: H256,
}

pub struct AttestationDataAndCustodyBit {
	/// Attestation data
	pub data: AttestationData,
	/// Custody bit
	pub custody_bit: bool,
}