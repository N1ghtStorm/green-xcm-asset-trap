// Copyright 2020-2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Pallet to handle XCM messages.

#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

use codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use frame_support::traits::{Contains, EnsureOrigin, Get, OriginTrait};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{BadOrigin, Saturating},
	RuntimeDebug,
};
use sp_std::{boxed::Box, marker::PhantomData, prelude::*, result::Result, vec};
use xcm::{latest::Weight as XcmWeight, prelude::*};
// use xcm_executor::traits::ConvertOrigin;

use frame_support::PalletId;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
		pallet_prelude::*,
		parameter_types,
	};
	use frame_system::{pallet_prelude::*, Config as SysConfig};
	use sp_core::H256;
	use sp_runtime::traits::{AccountIdConversion, BlakeTwo256, BlockNumberProvider, Hash};
	use xcm_executor::{
		traits::{
			ClaimAssets, DropAssets,
		},
		Assets,
	};

	// parameter_types! {
	// 	/// An implementation of `Get<u32>` which just returns the latest XCM version which we can
	// 	/// support.
	// 	pub const CurrentXcmVersion: u32 = XCM_VERSION;
	// }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	/// The module configuration trait.
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// /// Required origin for sending XCM messages. If successful, it resolves to `MultiLocation`
		// /// which exists as an interior location within this chain's XCM context.
		// type SendXcmOrigin: EnsureOrigin<
		// 	<Self as SysConfig>::RuntimeOrigin,
		// 	Success = MultiLocation,
		// >;

		// /// The type used to actually dispatch an XCM to its destination.
		// type XcmRouter: SendXcm;

		// /// Required origin for executing XCM messages, including the teleport functionality. If successful,
		// /// then it resolves to `MultiLocation` which exists as an interior location within this chain's XCM
		// /// context.
		// type ExecuteXcmOrigin: EnsureOrigin<
		// 	<Self as SysConfig>::RuntimeOrigin,
		// 	Success = MultiLocation,
		// >;

		// /// Our XCM filter which messages to be executed using `XcmExecutor` must pass.
		// type XcmExecuteFilter: Contains<(MultiLocation, Xcm<<Self as SysConfig>::RuntimeCall>)>;

		// /// Something to execute an XCM message.
		// type XcmExecutor: ExecuteXcm<<Self as SysConfig>::RuntimeCall>;

		// /// Our XCM filter which messages to be teleported using the dedicated extrinsic must pass.
		// type XcmTeleportFilter: Contains<(MultiLocation, Vec<MultiAsset>)>;

		// /// Our XCM filter which messages to be reserve-transferred using the dedicated extrinsic must pass.
		// type XcmReserveTransferFilter: Contains<(MultiLocation, Vec<MultiAsset>)>;

		// /// Means of measuring the weight consumed by an XCM message locally.
		// type Weigher: WeightBounds<<Self as SysConfig>::RuntimeCall>;

		// /// Means of inverting a location.
		// type LocationInverter: InvertLocation;

		// /// The outer `Origin` type.
		// type RuntimeOrigin: From<Origin> + From<<Self as SysConfig>::RuntimeOrigin>;

		// /// The outer `Call` type.
		// type RuntimeCall: Parameter
		// 	+ GetDispatchInfo
		// 	+ IsType<<Self as frame_system::Config>::RuntimeCall>
		// 	+ Dispatchable<
		// 		RuntimeOrigin = <Self as Config>::RuntimeOrigin,
		// 		PostInfo = PostDispatchInfo,
		// 	>;

		// const VERSION_DISCOVERY_QUEUE_SIZE: u32;

		// /// The latest supported version that we advertise. Generally just set it to
		// /// `pallet_xcm::CurrentXcmVersion`.
		// type AdvertisedXcmVersion: Get<XcmVersion>;
	}

	/// The maximum number of distinct assets allowed to be transferred in a single helper extrinsic.
	// const MAX_ASSETS_FOR_TRANSFER: usize = 2;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

	}

	#[pallet::error]
	pub enum Error<T> {

	}

	/// The existing asset traps.
	///
	/// Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of
	/// times this pair has been trapped (usually just 1 if it exists at all).
	#[pallet::storage]
	#[pallet::getter(fn asset_trap)]
	pub(super) type AssetTraps<T: Config> = StorageMap<_, Identity, H256, u32, ValueQuery>;

	// #[pallet::genesis_config]
	// pub struct GenesisConfig {
	// 	/// The default version to encode outgoing XCM messages with.
	// 	// pub safe_xcm_version: Option<XcmVersion>,
	// }

	// #[cfg(feature = "std")]
	// impl Default for GenesisConfig {
	// 	fn default() -> Self {
	// 		Self 
	// 	}
	// }

	// #[pallet::genesis_build]
	// impl<T: Config> GenesisBuild<T> for GenesisConfig {
	// 	fn build(&self) {
	// 		// SafeXcmVersion::<T>::set(self.safe_xcm_version);
	// 	}
	// }

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

	}

	impl<T: Config> Pallet<T> {

	}

	impl<T: Config> DropAssets for Pallet<T> {
		fn drop_assets(origin: &MultiLocation, assets: Assets) -> u64 {
			if assets.is_empty() {
				return 0;
			}
			// let versioned = VersionedMultiAssets::from(MultiAssets::from(assets));
			// let hash = BlakeTwo256::hash_of(&(&origin, &versioned));
			// AssetTraps::<T>::mutate(hash, |n| *n += 1);
			// Self::deposit_event(Event::AssetsTrapped(hash, origin.clone(), versioned));
			// TODO #3735: Put the real weight in there.
			0
		}
	}

	impl<T: Config> ClaimAssets for Pallet<T> {
		fn claim_assets(
			origin: &MultiLocation,
			ticket: &MultiLocation,
			assets: &MultiAssets,
		) -> bool {
			// let mut versioned = VersionedMultiAssets::from(assets.clone());
			// match (ticket.parents, &ticket.interior) {
			// 	(0, X1(GeneralIndex(i))) => {
			// 		versioned = match versioned.into_version(*i as u32) {
			// 			Ok(v) => v,
			// 			Err(()) => return false,
			// 		}
			// 	},
			// 	(0, Here) => (),
			// 	_ => return false,
			// };
			// let hash = BlakeTwo256::hash_of(&(origin, versioned.clone()));
			// match AssetTraps::<T>::get(hash) {
			// 	0 => return false,
			// 	1 => AssetTraps::<T>::remove(hash),
			// 	n => AssetTraps::<T>::insert(hash, n - 1),
			// }
			// Self::deposit_event(Event::AssetsClaimed(hash, origin.clone(), versioned));
			// return true;
			false
		}
	}
}
