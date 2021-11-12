#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
        dispatch::DispatchResult, 
        pallet_prelude::*,
        traits::{
			Currency, Get, LockIdentifier, LockableCurrency, WithdrawReasons,
        },
    };
	use frame_system::pallet_prelude::*;
	use micro_rand::*;
	use scale_info::prelude::vec::Vec;

	type BalanceOf<T> =
	<<T as Config>::HelloWorldCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	const LOCK_ID: LockIdentifier = *b"hllwrld ";
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The currency type for our pallet.
        type HelloWorldCurrency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    /// Keep track of a list of members authorized to play the game.
	#[pallet::storage]
	#[pallet::getter(fn members)]
	pub type Players<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		HelloWorld(T::AccountId),
        DepositLocked(T::AccountId, BalanceOf<T>),
        GameCompleted(T::AccountId, BalanceOf<T>),
		UnlockDeposit(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Call the hello_world function to sign up. 
		NotMember,
		/// Hello world can only be called once until the game is over.
		AlreadyMember,
	}

	// An account can register with this dispatchable to be able to play.
	// Accounts can register other accounts.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn hello_world(
            origin: OriginFor<T>,
			account: Vec<T::AccountId>, 
        ) -> DispatchResult {
            
            // Must be a signed call.
			let who = ensure_signed(origin)?;

            // Check that account isn't already registered.
            let members = <Players<T>>::get();
			let _location = members.binary_search(&who).err().ok_or(Error::<T>::AlreadyMember)?;

			<Players<T>>::put(account);

            // Log a message to the terminal.
            log::info!("🥳 Welcome to the party, {:?}", who);
			log::info!(" ");
        
            // Emit an event.
			Self::deposit_event(Event::HelloWorld(who));

			// Return a successful DispatchResultWithPostInfo.
			Ok(())
		}

		/// Allows a player to guess a number. 
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn play_game(
            origin: OriginFor<T>,
            deposit: BalanceOf<T>,
            guess: u32,
        ) -> DispatchResult {

            // Must be a signed call.
			let who = ensure_signed(origin)?;
            
            // Caller must be a member to play.
            let mut members = <Players<T>>::get();
            let location = members.binary_search(&who).ok().ok_or(Error::<T>::NotMember)?;
			members.insert(location, who.clone());
			let account_id = who.clone();

            // Lock deposit.
            T::HelloWorldCurrency::set_lock(LOCK_ID, &who, deposit, WithdrawReasons::empty());
            Self::deposit_event(Event::DepositLocked(who, deposit));
            log::info!("💰 {:?}, locked for {:?}", account_id, deposit);

            // Execute simple game logic.
            // Generate a random number and compare it with the player's guess.
            // If numbers match, player receives reward. Otherwise, player gets slashed. 
			let mut rnd = Random::new(1234);
			let get_random = rnd.next_int_i64(1, 5);
			let random_number: u32 = get_random as u32;

            // Earn or lose based on guess.
            if random_number == guess {
                T::HelloWorldCurrency::issue(deposit);
                T::HelloWorldCurrency::set_lock(LOCK_ID, &account_id, deposit, WithdrawReasons::all());
            } else {
                T::HelloWorldCurrency::slash(&account_id, random_number.into());
            }
			let total_balance = T::HelloWorldCurrency::total_balance(&account_id);
            
			log::info!("👀 👀 The number you guessed was {:?} ", guess);
			log::info!("The number generated was {:?} ", random_number);
            log::info!("📣 📣 Your total balance is {:?} ", total_balance);

			// Deposit GameCompleted event.			
			Self::deposit_event(Event::GameCompleted(account_id, T::HelloWorldCurrency::total_issuance()));

			Ok(().into())
		}
	
		/// Allows a player to unlock their deposit and receive their rewards.
		#[pallet::weight(10_000)]
		pub fn unlock_deposit(
			origin: OriginFor<T>,
		) -> DispatchResult {

		// Must be a signed call.
		let who = ensure_signed(origin)?;
		
		// Caller must be a member to play.
		let mut members = <Players<T>>::get();
		let location = members.binary_search(&who).ok().ok_or(Error::<T>::NotMember)?;

		let account_id = who.clone();

		// Remove member from list.
		members.remove(location);
		
		// Give deposit back. 
		T::HelloWorldCurrency::remove_lock(LOCK_ID, &account_id);

		// Deposit UnlockDeposit event.			
		Self::deposit_event(Event::UnlockDeposit(account_id));

		Ok(().into())

		}
	}
}