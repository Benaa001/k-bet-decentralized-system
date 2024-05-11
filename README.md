## Decentralized Betting System Documentation

### Overview
The Decentralized Betting System is a web-based application designed to facilitate betting on various games in a decentralized manner. It provides functionalities for placing bets, creating pools, scheduling games, and releasing funds based on game outcomes. The system aims to provide a transparent and secure platform for users to participate in betting activities.

The application is built using Rust programming language with the Internet Computer (IC) Canister SDK, ensuring secure and decentralized management of bets and game-related activities. It leverages stable data structures for efficient storage and retrieval of data, providing a reliable platform for users to engage in betting.

### Table of Contents
1. [Dependencies](#dependencies)
2. [Data Structures](#data-structures)
3. [Functions](#functions)
4. [Usage](#usage)
5. [Setting Up the Project](#setup)

### Dependencies <a name="dependencies"></a>
- `serde`: Serialization and deserialization library for Rust.
- `candid`: Library for Candid serialization and deserialization.
- `ic_stable_structures`: Library providing stable data structures for the Internet Computer.
- `std`: Standard library for Rust.

### Data Structures <a name="data-structures"></a>
#### Structs
1. `Bet`: Represents a bet with fields such as ID, user ID, amount, game ID, chosen outcome, and timestamp.
2. `User`: Represents a user with fields including ID, name, and balance.
3. `Pool`: Represents a pool with fields including ID, game ID, and total amount.
4. `Game`: Represents a game with fields including ID, name, start time, and end time.
5. `Results`: Represents the results of a game with fields including ID, game ID, outcome, and timestamp.
6. `Escrow`: Represents an escrow with fields including ID, game ID, amount, and bet ID.

#### Enums
1. `GameOutcome`: Represents the possible outcomes for a game including Win, Loss, and Draw.

### Functions <a name="functions"></a>
The Decentralized Betting System provides various functions for managing bets, games, and user information. Some key functions include:
- `place_bet`: Place a new bet.
- `schedule_game`: Schedule a new game.
- `create_pool`: Create a new pool for a game.
- `add_user`: Add a new user.
- `add_results`: Add results for a game.
- `release_funds`: Release funds based on game outcomes.
- `get_user`: Get user information by ID.
- `get_game`: Get game information by ID.
- `get_bet`: Get bet information by ID.

### Usage <a name="usage"></a>
The Decentralized Betting System offers a user-friendly interface for users to interact with the platform. Users can place bets on available games, view game schedules, and track their betting history. Administrators can manage games, pools, and user information.

To use the application, users can navigate through the interface, place bets on desired games, and view their winnings based on game outcomes. Proper error handling is implemented to handle cases such as insufficient balance or invalid input.

### Setting Up the Project <a name="setup"></a>
To set up and start working on the Decentralized Betting System project, follow these steps:

1. **Install Rust and Dependencies**
   - Ensure you have Rust installed, version 1.64 or higher. You can install it using the following commands:
     ```bash
     $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
     $ source "$HOME/.cargo/env"
     ```
   - Install the `wasm32-unknown-unknown` target:
     ```bash
     $ rustup target add wasm32-unknown-unknown
     ```
   - Install `candid-extractor`:
     ```bash
     $ cargo install candid-extractor
     ```

2. **Install DFINITY SDK (`dfx`)**
   - Install `dfx` using the following commands:
     ```bash
     $ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
     $ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
     $ source ~/.bashrc
     $ dfx start --background
     ```

3. **Update Dependencies**
   - Update the `dependencies` block in your `Cargo.toml` file with the required dependencies.

4. **Generate Candid Interface**
   - Use the `candid` tool to generate the Candid interface for your canister:
     ```bash
     $ candid generate <your_canister_name>.did
     ```

5. **Start the Project Locally**
   - Start the replica, running in the background:
     ```bash
     $ dfx start --background
     ```
   - Deploy your canisters to the replica and generate your Candid interface:
     ```bash
     $ npm run gen-deploy
     ```

6. **Interact with the System**
   - You can now interact with the Decentralized Betting System through the provided interface.

