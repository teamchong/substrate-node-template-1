#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_support::traits::Box;
use frame_system::ensure_signed;
use frame_support::weights::Pays;
use frame_support::Parameter;
use frame_support::dispatch::{Dispatchable, DispatchResult};
use frame_support::weights::GetDispatchInfo;
use frame_support::pallet_prelude::DispatchResultWithPostInfo;


/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type Call: Parameter + Dispatchable<Origin=Self::Origin> + GetDispatchInfo;
	
}

type Count = u32;

decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------
	trait Store for Module<T: Config> as TemplateModule {
		Tracker : map hasher(twox_64_concat) T::AccountId => (T::BlockNumber, Count);
		 MaxCalls: Count = 100;
		 SessionLength: T::BlockNumber; 
		}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		ExtrinsicResult(AccountId, DispatchResult),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {

		// User maxed out their calls
		NoFreeCalls,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = {
			let dispatch_info = call.get_dispatch_info();
			(dispatch_info.weight, dispatch_info.class, Pays::Yes)
		}]

		fn do_this(origin, call: Box<<T as Config>::Call>) -> DispatchResultWithPostInfo {
			// Ensures the sender is from origin
			let sender = ensure_signed(origin.clone())?;

			// set things up to keep track of MaxCalls and Tracker
			let max_calls = MaxCalls::get();
			let (last_user_session, mut user_calls) = Tracker::<T>::get(&sender);

			// get current block number
			let current_block_number = frame_system::Module::<T>::block_number();

			let session_length = SessionLength::<T>::get();
			// session is 4 if your on block 4000-4999
			let current_session = current_block_number / session_length;

			if (last_user_session < current_session) {
				user_calls = 0;
			}
 
			// sanity check
			// ensure!(user_calls < max_calls, Error::<T>::NoFreeCalls);
			
		if user_calls < max_calls {
		
			Tracker::<T>::insert(				
				&sender, 
				(
					current_session, 
					user_calls.saturating_add(1),
				)
			);

			let result = call.dispatch(origin);

			Self::deposit_event(
				RawEvent::ExtrinsicResult(
					sender, 
					result.map(|_| ()).map_err(|e| e.error),
					)
				);
				return Ok(Pays::No.into())

		} else {
			let check_logic_weight = T::DbWeight::get().reads(3);
			return Ok(Some(check_logic_weight).into())
		}
			
		
			// add a note by add a note: Vec<u8>
			// deposit_event(RawEvent::Noted(note));
			let result = call.dispatch(origin);

			// // we want to calculate our new weight and reduce the txn fee
			// let new_weight = call.get_dispatch_info().weight - 100;
			Ok(Pays::No.into())
			// Ok(Some(new_weight.into()))
		}
	}
}
