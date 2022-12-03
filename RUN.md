# Running the App

## Setup: Compile constants
Run `cargo build --release` in constants/.

## Running the Client
- Start by adding wasm as a rust target: `rustup target add wasm32-unknown-unknown`.
- Install trunk: `cargo install trunk`.
- Run the Yew app in yew-app/: `trunk serve`. This will run the app on port 8080; go to `localhost:8080` to see the app.

## Running the Server
To play 'online' between local clients, start the server with `cargo run --release` in server/. Then navigate to "Online Multiplayer" in the web app from both clients. Be sure to join the same lobby (or don't specify a lobby for the default).