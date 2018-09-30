#![cfg_attr(not(feature = "std"), no_std)]

extern crate blake2;
extern crate parity_codec as codec;
#[macro_use]
extern crate parity_codec_derive;
extern crate ssz;
#[macro_use]
extern crate ssz_derive;
extern crate hashdb;
extern crate plain_hasher;
extern crate tiny_keccak;
extern crate bls;
extern crate bls_aggregates;
extern crate shuffling;
extern crate byteorder;

#[cfg(feature = "std")]
extern crate serde;

#[cfg(feature = "std")]
#[macro_use]
extern crate serde_derive;

extern crate substrate_primitives as primitives;
extern crate sr_std as rstd;
extern crate sr_primitives as runtime_primitives;
#[macro_use]
extern crate sr_io as runtime_io;
#[macro_use]
extern crate sr_version as runtime_version;
#[macro_use]
extern crate srml_support as runtime_support;

mod attestation;
mod extrinsic;
mod header;
mod utils;
mod state;
mod system;
mod validators;
mod block;
mod bitfield;

pub mod consts;
pub mod spec;
pub mod validation;

pub use attestation::AttestationRecord;
pub use header::{Header, Digest, DigestItem};
pub use extrinsic::Extrinsic;
pub use state::{CrosslinkRecord, ActiveState, CrystallizedState};
pub use validators::{ValidatorRecord, ShardAndCommittee};
pub use block::{Block, BlockId, BlockExt};
pub use bitfield::BitField;

use primitives::{H256, H160};
use rstd::prelude::*;

use runtime_version::{RuntimeVersion, NativeVersion};

/// Shasper runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: ver_str!("shasper"),
	impl_name: ver_str!("parity-shasper"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: apis_vec!([]),
};

#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

pub type Hash = H256;
pub type BlockNumber = u64;
pub type Address = H160;
pub type PublicKey = Vec<u8>;
pub type ShardId = u16;
pub type InherentData = ();

pub mod api {
	use system;
	impl_stubs!(
		initialise_block => |header| system::initialise_block(header),
		apply_extrinsic => |extrinsic| system::apply_extrinsic(extrinsic),
		finalise_block => |()| system::finalise_block(),
		inherent_extrinsics => |_| system::inherent_extrinsics(),
		execute_block => |block| system::execute_block(block),
		active_state_root => |()| system::active_state_root(),
		crystallized_state_root => |()| system::crystallized_state_root(),
		authorities => |()| system::authorities(),
		version => |()| ::VERSION
	);
}
