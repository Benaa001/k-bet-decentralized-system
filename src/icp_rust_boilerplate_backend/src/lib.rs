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
    bet_history: Vec<u64>, // List of bet IDs for bet history
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

// Thread-local storage initialization
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

macro_rules! impl_storable {
    ($type:ty, $max_size:expr) => {
        impl Storable for $type {
            fn to_bytes(&self) -> Cow<[u8]> {
                Cow::Owned(Encode!(self).unwrap())
            }

            fn from_bytes(bytes: Cow<[u8]>) -> Self {
                Decode!(bytes.as_ref(), Self).unwrap()
            }
        }

        impl BoundedStorable for $type {
            const MAX_SIZE: u32 = $max_size;
            const IS_FIXED_SIZE: bool = false;
        }
    };
}

impl_storable!(Bet, 1024);
impl_storable!(User, 2048);
impl_storable!(Pool, 1024);
impl_storable!(Game, 1024);
impl_storable!(Escrow, 1024);
impl_storable!(Results, 1024);

// Define update and query methods for interacting with the system

#[ic_cdk::update]
fn add_user(name: String, balance: u64) -> Result<User, Error> {
    let id = generate_unique_id();
    let user = User {
        id,
        name,
        balance,
        bet_history: Vec::new(),
    };
    USER_STORAGE.with(|storage| storage.borrow_mut().insert(id, user.clone()));
    Ok(user)
}

#[ic_cdk::update]
fn create_pool(game_id: u64, total_amount: u64) -> Result<Pool, Error> {
    let id = generate_unique_id();
    let pool = Pool {
        id,
        game_id,
        total_amount,
    };
    POOL_STORAGE.with(|storage| storage.borrow_mut().insert(id, pool.clone()));
    Ok(pool)
}

#[ic_cdk::update]
fn add_game(name: String, start_time: u64, end_time: u64) -> Result<Game, Error> {
    let id = generate_unique_id();
    let game = Game {
        id,
        name,
        start_time,
        end_time,
    };
    GAME_STORAGE.with(|storage| storage.borrow_mut().insert(id, game.clone()));
    Ok(game)
}

#[ic_cdk::update]
fn add_bet(user_id: u64, amount: u64, game_id: u64, chosen_outcome: GameOutcome) -> Result<Bet, Error> {
    let id = generate_unique_id();
    let timestamp = ic_cdk::api::time() as u64;
    let bet = Bet {
        id,
        user_id,
        amount,
        game_id,
        chosen_outcome,
        timestamp,
    };
    BET_STORAGE.with(|storage| storage.borrow_mut().insert(id, bet.clone()));
    Ok(bet)
}

#[ic_cdk::update]
fn create_escrow(game_id: u64, amount: u64, bet_id: u64) -> Result<Escrow, Error> {
    let id = generate_unique_id();
    let escrow = Escrow {
        id,
        game_id,
        amount,
        bet_id,
    };
    ESCROW_STORAGE.with(|storage| storage.borrow_mut().insert(id, escrow.clone()));
    Ok(escrow)
}

// Define query methods

#[ic_cdk::query]
fn get_user(id: u64) -> Result<User, Error> {
    match USER_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(user) => Ok(user.clone()),
        None => Err(Error::NotFound {
            msg: format!("User with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_pool(id: u64) -> Result<Pool, Error> {
    match POOL_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(pool) => Ok(pool.clone()),
        None => Err(Error::NotFound {
            msg: format!("Pool with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_game(id: u64) -> Result<Game, Error> {
    match GAME_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(game) => Ok(game.clone()),
        None => Err(Error::NotFound {
            msg: format!("Game with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_escrow(id: u64) -> Result<Escrow, Error> {
    match ESCROW_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(escrow) => Ok(escrow.clone()),
        None => Err(Error::NotFound {
            msg: format!("Escrow with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_bet(id: u64) -> Result<Bet, Error> {
    match BET_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(bet) => Ok(bet.clone()),
        None => Err(Error::NotFound {
            msg: format!("Bet with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_results(game_id: u64) -> Result<Results, Error> {
    match RESULTS_STORAGE.with(|storage| storage.borrow().get(&game_id)) {
        Some(results) => Ok(results.clone()),
        None => Err(Error::NotFound {
            msg: format!("Results for game with id={} not found", game_id),
        }),
    }
}

// Define update methods for placing bets and managing balances

#[ic_cdk::update]
fn place_bet(user_id: u64, amount: u64, game_id: u64, chosen_outcome: GameOutcome) -> Result<Bet, Error> {
    let current_time = ic_cdk::api::time() as u64;

    // Check if the user exists and has enough balance
    let mut user = get_user(user_id)?;
    if amount > user.balance {
        return Err(Error::InvalidInput {
            msg: "Insufficient balance to place the bet".to_string(),
        });
    }

    // Check if the game exists and is open for betting
    let game = get_game(game_id)?;
    if current_time >= game.end_time {
        return Err(Error::InvalidInput {
            msg: "Betting for this game has ended".to_string(),
        });
    }

    // Deduct the bet amount from the user's balance
    user.balance -= amount;
    USER_STORAGE.with(|storage| storage.borrow_mut().insert(user_id, user.clone()));

    // Create and store the bet
    let bet = add_bet(user_id, amount, game_id, chosen_outcome)?;

    // Add bet ID to user's bet history
    user.bet_history.push(bet.id);
    USER_STORAGE.with(|storage| storage.borrow_mut().insert(user_id, user.clone()));

    // Create escrow for the bet amount
    create_escrow(game_id, amount, bet.id)?;

    Ok(bet)
}

#[ic_cdk::update]
fn release_funds(game_id: u64) -> Result<(), Error> {
    // Get the results of the game
    let results = get_results(game_id)?;

    // Get all escrows related to this game
    let escrows: Vec<Escrow> = ESCROW_STORAGE.with(|storage| {
        let storage = storage.borrow();
        storage.iter().filter(|(_, escrow)| escrow.game_id == game_id).map(|(_, escrow)| escrow.clone()).collect()
    });

    // Update balances based on the results
    for escrow in escrows {
        // Get the bet related to this escrow
        let bet = get_bet(escrow.bet_id)?;
        // Check if the bet outcome matches the game outcome
        if bet.chosen_outcome == results.outcome {
            // Update user balance if the bet was successful
            let mut user = get_user(bet.user_id)?;
            user.balance += escrow.amount;
            USER_STORAGE.with(|storage| storage.borrow_mut().insert(user.id, user.clone()));
        }
        // Delete the escrow
        ESCROW_STORAGE.with(|storage| storage.borrow_mut().remove(&escrow.id));
    }

    Ok(())
}

// Helper function to generate unique IDs for entities
fn generate_unique_id() -> u64 {
    ID_COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        let id = *counter.get();
        counter.set(id + 1).unwrap();
        id
    })
}

// Define Candid interface

ic_cdk::export_candid!();

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_unique_id() {
        let id1 = generate_unique_id();
        let id2 = generate_unique_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_add_user() {
        let user = add_user("Alice".to_string(), 1000).unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(user.balance, 1000);
    }
}
