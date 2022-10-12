#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use codec::Encode;
	use frame_support::{
		inherent::Vec, pallet_prelude::*, sp_runtime::traits::Hash, traits::Randomness,
	};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;

	// Type aliases

	type WorkshopId<T> = <T as frame_system::Config>::Hash;
	type WorkshopName<T> = BoundedVec<u8, <T as Config>::MaxWorkshopNameLength>;
	type WorkshopOwner<T> = <T as frame_system::Config>::AccountId;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type MaxWorkshopNameLength: Get<u32>;

		type Rand: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::storage]
	#[pallet::getter(fn workshops)]
	pub type Workshops<T> =
		StorageMap<_, Blake2_128Concat, WorkshopId<T>, (WorkshopName<T>, WorkshopOwner<T>)>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		WorkshopCreated { id: WorkshopId<T>, name: WorkshopName<T>, owner: WorkshopOwner<T> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		WorkshopNameTooLong,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_workshop(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let workshop_id = T::Hashing::hash_of(&Self::gen_id());

			let workshop_name: WorkshopName<T> =
				name.try_into().map_err(|_| Error::<T>::WorkshopNameTooLong)?;

			// Update storage.
			Workshops::<T>::insert(workshop_id, (workshop_name.clone(), who.clone()));

			// Emit an event.
			Self::deposit_event(Event::WorkshopCreated {
				id: workshop_id,
				name: workshop_name,
				owner: who,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn gen_id() -> [u8; 16] {
			let payload =
				(T::Rand::random(&b"id"[..]).0, <frame_system::Pallet<T>>::block_number());
			payload.using_encoded(blake2_128)
		}
	}
}
