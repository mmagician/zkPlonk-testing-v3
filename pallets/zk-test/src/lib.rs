#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
//! A simple pallet with two storage values. The pallet itself does not teach any new concepts.
//! Rather we use this pallet as demonstration case as we demonstrate custom runtime APIs.
//! This pallet supports a runtime API which will allow querying the runtime for the sum of
//! the two storage items.

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    pub use plonk_pallet::{ParityRng, Proof, PublicInputValue, Transcript, VerifierData};
    use sp_std::vec::Vec;

    /// Copuliing configuration trait with plonk_pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config + plonk_pallet::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(u32 = "Metadata")]
    pub enum Event<T: Config> {
        ValueSet(u32, u32),
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn thing1)]
    pub type Thing1<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn thing2)]
    pub type Thing2<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // The module's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Coupled trusted setup
        #[pallet::weight(10_000)]
        pub fn trusted_setup(
            origin: OriginFor<T>,
            val: u32,
            rng: ParityRng,
        ) -> DispatchResultWithPostInfo {
            plonk_pallet::Pallet::<T>::trusted_setup(origin, val, rng)?;
            Ok(().into())
        }

        /// Sets the first simple storage value
        #[pallet::weight(10_000)]
        pub fn set_thing_1(
            origin: OriginFor<T>,
            val: u32,
            vd: VerifierData,
            proof: Proof,
            public_inputs: Vec<PublicInputValue>,
            transcript_init: Transcript,
        ) -> DispatchResultWithPostInfo {
            // Define the proof varification
            plonk_pallet::Pallet::<T>::verify(origin, vd, proof, public_inputs, transcript_init)?;

            Thing1::<T>::put(val);

            Self::deposit_event(Event::ValueSet(1, val));
            Ok(().into())
        }

        /// Sets the second stored value
        #[pallet::weight(10_000)]
        pub fn set_thing_2(origin: OriginFor<T>, val: u32) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            Thing2::<T>::put(val);

            Self::deposit_event(Event::ValueSet(2, val));
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn get_sum() -> u32 {
        Thing1::<T>::get() + Thing2::<T>::get()
    }
}
