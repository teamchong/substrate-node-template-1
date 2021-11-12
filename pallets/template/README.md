License: Unlicense

# Hello world game

This pallet is meant to be the basis of some educational material to write a `hello_world` dispatchable that emits a `HelloWorld` event.
Additionally, it contains a disptachable that gives registered players the ability to guess a number and receive a reward if they guess correctly.

## Functionality

The `hello_world_game` pallet contains two dispatchables:

1. `hello_world` - this allows anyone to add any account to register to play a game.
1. `play_game` - this checks whether the caller's account ID is in the list of members and allows the caller to enter a number. 
    If the number matches the random number that's generated, the player doubles their deposit. If it doesn't match, they lose their deposit.
1. `unlock_earnings` - this allows a player to unlock their funds and get their deposit back. 


## Showcase

- `HelloWorldCurrency` - the `Currency` type for the pallet, used to lock deposits.
- `BalanceOf<T>` - handles changes in account balances, using the `Balnce` type from `frame_system`.
- `log::info!` - put messages in the terminal, useful for debugging. 
- `Players<T>` - a storage item that stores a `Vec` of account IDs who are authorized to play the guessing game.  
- `Events` and `Errors` - emits various events and errors.

