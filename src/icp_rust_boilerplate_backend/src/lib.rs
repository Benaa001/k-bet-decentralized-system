// Decentralized Betting System Implementation

#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define memory types and storage
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define data structures

#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Bet {
    id: u64,
    user_id: u64,
    amount: u64,
    game_id: u64,
    chosen_outcome: GameOutcome,
    timestamp: u64,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct User {
    id: u64,
    name: String,
    balance: u64,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Pool {
    id: u64,
    game_id: u64,
    total_amount: u64,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Game {
    id: u64,
    name: String,
    start_time: u64,
    end_time: u64,
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Results {
    id: u64,
    game_id: u64,
    outcome: GameOutcome,
    timestamp: u64,
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Escrow {
    id: u64,
    game_id: u64,
    amount: u64,
    bet_id: u64,
}

#[derive(candid::CandidType, PartialEq, Serialize, Deserialize, Clone, Copy)]
enum GameOutcome {
    Win,
    Loss,
    Draw,
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    // Define storage for different entities

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static BET_STORAGE: RefCell<StableBTreeMap<u64, Bet, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static POOL_STORAGE: RefCell<StableBTreeMap<u64, Pool, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static GAME_STORAGE: RefCell<StableBTreeMap<u64, Game, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static ESCROW_STORAGE: RefCell<StableBTreeMap<u64, Escrow, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

    static RESULTS_STORAGE: RefCell<StableBTreeMap<u64, Results, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));
}

// Define errors

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
}

// Implement Storable and BoundedStorable traits for data structures

impl Storable for Bet {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Bet {
    const MAX_SIZE: u32 = 1024; // Adjust the maximum size as needed
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024; // Adjust the maximum size as needed
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Pool {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Pool {
    const MAX_SIZE: u32 = 1024; // Adjust the maximum size as needed
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Game {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Game {
    const MAX_SIZE: u32 = 1024; // Adjust the maximum size as needed
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Escrow {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Escrow {
    const MAX_SIZE: u32 = 1024; // Adjust the maximum size as needed
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Results {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Results {
    const MAX_SIZE: u32 = 1024; // Adjust the maximum size as needed
    const IS_FIXED_SIZE: bool = false;
}

// Define update and query methods for interacting with the system

#[ic_cdk::update]
fn add_user(id: u64, name: String, balance: u64) -> Result<User, Error> {
    let user = User { id, name, balance };
    USER_STORAGE.with(|service| service.borrow_mut().insert(id, user.clone()));
    Ok(user)
}

#[ic_cdk::update]
fn create_pool(id: u64, game_id: u64, total_amount: u64) -> Result<Pool, Error> {
    let pool = Pool { id, game_id, total_amount };
    POOL_STORAGE.with(|service| service.borrow_mut().insert(id, pool.clone()));
    Ok(pool)
}

#[ic_cdk::update]
fn add_game(id: u64, name: String, start_time: u64, end_time: u64) -> Result<Game, Error> {
    let game = Game { id, name, start_time, end_time };
    GAME_STORAGE.with(|service| service.borrow_mut().insert(id, game.clone()));
    Ok(game)
}

#[ic_cdk::update]
fn add_bet(id: u64, user_id: u64, amount: u64, game_id: u64, chosen_outcome: GameOutcome, timestamp: u64) -> Result<Bet, Error> {
    let bet = Bet { id, user_id, amount, game_id, chosen_outcome, timestamp };
    BET_STORAGE.with(|service| service.borrow_mut().insert(id, bet.clone()));
    Ok(bet)
}

#[ic_cdk::update]
fn create_escrow(id: u64, game_id: u64, amount: u64, bet_id: u64) -> Result<Escrow, Error> {
    let escrow = Escrow { id, game_id, amount, bet_id };
    ESCROW_STORAGE.with(|service| service.borrow_mut().insert(id, escrow.clone()));
    Ok(escrow)
}

// Define query methods

#[ic_cdk::query]
fn get_user(id: u64) -> Result<User, Error> {
    match USER_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(user) => Ok(user.clone()),
        None => Err(Error::NotFound {
            msg: format!("User with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_pool(id: u64) -> Result<Pool, Error> {
    match POOL_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(pool) => Ok(pool.clone()),
        None => Err(Error::NotFound {
            msg: format!("Pool with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_game(id: u64) -> Result<Game, Error> {
    match GAME_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(game) => Ok(game.clone()),
        None => Err(Error::NotFound {
            msg: format!("Game with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_escrow(id: u64) -> Result<Escrow, Error> {
    match ESCROW_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(escrow) => Ok(escrow.clone()),
        None => Err(Error::NotFound {
            msg: format!("Escrow with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn schedule_game(id: u64, name: String, start_time: u64, end_time: u64) -> Result<Game, Error> {
    let current_time = ic_cdk::api::time() as u64;
    if start_time < current_time || end_time < current_time || end_time <= start_time {
        return Err(Error::InvalidInput {
            msg: "Invalid start or end time".to_string(),
        });
    }

    let game = Game { id, name, start_time, end_time };
    GAME_STORAGE.with(|service| service.borrow_mut().insert(id, game.clone()));

    Ok(game)
}

// Define update and query methods for interacting with the system

#[ic_cdk::update]
fn add_results(id: u64, game_id: u64, outcome: GameOutcome, timestamp: u64) -> Result<Results, Error> {
    let results = Results { id, game_id, outcome, timestamp };
    RESULTS_STORAGE.with(|service| service.borrow_mut().insert(id, results.clone()));
    Ok(results)
}

#[ic_cdk::query]
fn get_results(id: u64) -> Result<Results, Error> {
    match RESULTS_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(results) => Ok(results.clone()),
        None => Err(Error::NotFound {
            msg: format!("Results with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_bet(id: u64) -> Result<Bet, Error> {
    match BET_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(bet) => Ok(bet.clone()),
        None => Err(Error::NotFound {
            msg: format!("Bet with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn update_user(id: u64, name: String, balance: u64) -> Result<User, Error> {
    let updated_user = User { id, name, balance };
    match USER_STORAGE.with(|service| service.borrow_mut().insert(id, updated_user.clone())) {
        Some(_) => Ok(updated_user),
        None => Err(Error::NotFound {
            msg: format!("User with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_user(id: u64) -> Result<(), Error> {
    match USER_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("User with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn update_bet(id: u64, user_id: u64, amount: u64, game_id: u64, chosen_outcome: GameOutcome, timestamp: u64) -> Result<Bet, Error> {
    let updated_bet = Bet { id, user_id, amount, game_id, chosen_outcome, timestamp };
    match BET_STORAGE.with(|service| service.borrow_mut().insert(id, updated_bet.clone())) {
        Some(_) => Ok(updated_bet),
        None => Err(Error::NotFound {
            msg: format!("Bet with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_bet(id: u64) -> Result<(), Error> {
    match BET_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Bet with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn update_pool(id: u64, game_id: u64, total_amount: u64) -> Result<Pool, Error> {
    let updated_pool = Pool { id, game_id, total_amount };
    match POOL_STORAGE.with(|service| service.borrow_mut().insert(id, updated_pool.clone())) {
        Some(_) => Ok(updated_pool),
        None => Err(Error::NotFound {
            msg: format!("Pool with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_pool(id: u64) -> Result<(), Error> {
    match POOL_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Pool with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn update_game(id: u64, name: String, start_time: u64, end_time: u64) -> Result<Game, Error> {
    let updated_game = Game { id, name, start_time, end_time };
    match GAME_STORAGE.with(|service| service.borrow_mut().insert(id, updated_game.clone())) {
        Some(_) => Ok(updated_game),
        None => Err(Error::NotFound {
            msg: format!("Game with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_game(id: u64) -> Result<(), Error> {
    match GAME_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Game with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn update_escrow(id: u64, game_id: u64, amount: u64, bet_id: u64) -> Result<Escrow, Error> {
    let updated_escrow = Escrow { id, game_id, amount, bet_id };
    match ESCROW_STORAGE.with(|service| service.borrow_mut().insert(id, updated_escrow.clone())) {
        Some(_) => Ok(updated_escrow),
        None => Err(Error::NotFound {
            msg: format!("Escrow with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_escrow(id: u64) -> Result<(), Error> {
    match ESCROW_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Escrow with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn update_results(id: u64, game_id: u64, outcome: GameOutcome, timestamp: u64) -> Result<Results, Error> {
    let updated_results = Results { id, game_id, outcome, timestamp };
    match RESULTS_STORAGE.with(|service| service.borrow_mut().insert(id, updated_results.clone())) {
        Some(_) => Ok(updated_results),
        None => Err(Error::NotFound {
            msg: format!("Results with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_results(id: u64) -> Result<(), Error> {
    match RESULTS_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Results with id={} not found", id),
        }),
    }
}

// Define update method for placing a bet
#[ic_cdk::update]
fn place_bet(user_id: u64, amount: u64, game_id: u64, chosen_outcome: GameOutcome) -> Result<Bet, Error> {
    // Check if the user exists
    let user = get_user(user_id)?;
    
    // Check if the game exists
    let game = get_game(game_id)?;
    
    // Check if the game is still open for betting
    let current_time = ic_cdk::api::time() as u64;
    if current_time <= game.end_time {
        return Err(Error::InvalidInput {
            msg: "Betting for this game has ended".to_string(),
        });
    }
    
    // Ensure the user has sufficient balance
    if amount > user.balance {
        return Err(Error::InvalidInput {
            msg: "Insufficient balance to place the bet".to_string(),
        });
    }
    
    // Deduct the bet amount from the user's balance
    let updated_balance = user.balance - amount;
    update_user(user_id, user.name.clone(), updated_balance)?;
    
    // Generate a unique ID for the bet
    let bet_id = generate_unique_id();
    
    // Create the bet object
    let timestamp = current_time;
    let bet = Bet {
        id: bet_id,
        user_id,
        amount,
        game_id,
        chosen_outcome,
        timestamp,
    };
    
    // Store the bet in the storage
    add_bet(bet_id, user_id, amount, game_id, chosen_outcome, timestamp)?;
    
    Ok(bet)
}

// Helper function to generate unique IDs for bets
fn generate_unique_id() -> u64 {
    // Generate a unique ID using a combination of timestamp and random number
    let timestamp = ic_cdk::api::time() as u64;
    let random_number: u64 = ic_cdk::api::time() as u64; // You may replace this with a better random number generator
    (timestamp << 32) | (random_number & 0xFFFF_FFFF)
}

#[ic_cdk::update]
fn release_funds(game_id: u64) -> Result<(), Error> {
    // Get the results of the game
    let results = get_results(game_id)?;
    
    // Get all escrows related to this game
    let escrows = ESCROW_STORAGE.with(|storage| {
        let storage = storage.borrow();
        storage.iter().filter(|(_, escrow)| escrow.game_id == game_id).map(|(_, escrow)| escrow.clone()).collect::<Vec<_>>()
    });
    
    // Update balances based on the results
    for escrow in escrows {
        // Get the bet related to this escrow
        let bet = get_bet(escrow.bet_id)?;
        // Check if the bet outcome matches the game outcome
        if bet.chosen_outcome == results.outcome {
            // Update user balance if the bet was successful
            let user = get_user(bet.user_id)?;
            let updated_balance = user.balance + escrow.amount;
            update_user(user.id, user.name, updated_balance)?;
        }
        // Delete the escrow
        delete_escrow(escrow.id)?;
    }
    
    Ok(())
}

// Define Candid interface

ic_cdk::export_candid!();

