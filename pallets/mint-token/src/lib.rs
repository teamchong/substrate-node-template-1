#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::{self as system, ensure_signed};
use frame_support::pallet_prelude::Get;

// Modify the Config trait to include the Instance as a type parameter
pub trait Config<I: Instance>: frame_system::Config {
	type Event: From<Event<Self, I>> + Into<<Self as frame_system::Config>::Event>;
	type Supply: Get<u64>;
}


// Include the I: Instance type parameter in storage declaration
decl_storage! {
	trait Store for Module<T: Config<I>, I: Instance> as MintToken {
		Balances: map hasher(blake2_128_concat) T::AccountId => u64;
	}
}

// The enum trait also takes the Instance as a parameter
decl_event!(
	pub enum Event<T, I> where AccountId = <T as frame_system::Config>::AccountId {
		// Token was initialized by user
		Initialized(AccountId),
		// Tokens successfully transferred between users
		Transfer(AccountId, AccountId, u64), // (from, to, value)
	}

);

decl_error! {
	pub enum Error for Module<T: Config<I>, I: Instance> {
		/// Attempted to transfer more funds than were available
		InsufficientFunds,
	}
}

// Include I: Instance for the Module struct
decl_module! {
	pub struct Module<T: Config<I>, I: Instance> for enum Call where origin: T::Origin {
		
		fn deposit_event() = default;

		// Initializes the token and
		// transfers the total_supply amout to the caller
		#[weight = 10_000]
		fn init(origin) -> DispatchResult {
			
			// define the account initializing the token mint 
			let sender = ensure_signed(origin)?;
		
			// send the total supply created to the origin account using the balances pallet
			
			// When writing to storage, we supply, not only a configuration T, but also an
			// instance, I.

			<Balances<T, I>>::insert(sender, T::Supply::get());
			
			Ok(())
		}
		
		// Transfer tokens from one account to another
		#[weight = 10_000]
		fn transfer(_origin, to: T::AccountId, value: u64) -> DispatchResult {
			let sender = ensure_signed(_origin)?;
			let sender_balance = Balances::<T, I>::get(&sender);
			let receiver_balance = Balances::<T, I>::get(&to);

			// Calculate new balances
			let updated_from_balance = sender_balance.checked_sub(value).ok_or(<Error<T, I>>::InsufficientFunds)?;
			let updated_to_balance = receiver_balance.checked_add(value).expect("Entire supply fits in u64; qed");

			// Write new balances to storage
			<Balances<T, I>>::insert(&sender, updated_from_balance);
			<Balances<T, I>>::insert(&to, updated_to_balance);
			
			Self::deposit_event(RawEvent::Transfer(sender, to, value));
			Ok(())
		}

	}
}
