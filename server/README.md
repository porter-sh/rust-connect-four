# Server

The mediator of online play. Communicates with the yew-app client (or any client that follows proper protocols for that matter) to allow online matches.

## Run the Server

Run `cargo run --release` in server/

## C++ Integration

### Setup

To use a C++ inplementation of a Connect Four board with the server, put a board.cc file into the server/cpplib/ directory. Alter the board.hpp function with forward declarations for helper functions if necessary (or move those forward declarations in board.cc).

### Building

In Cargo.toml uncomment the build-dependencies section. Change build.rs.txt to build.rs, and uncomment / add compilation flags to the method chain as needed for your system.

Now to have the server use the C++ integration, activate the `cppintegration` feature (includ it in the default feature list) in Cargo.toml, and run the server.