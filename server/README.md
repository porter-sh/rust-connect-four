# Server
The mediator of online play. Communicates with the yew-app client (or any client that follows proper protocols for that matter) to allow online matches.

## Run the Server
Run `cargo run --release` in server/

### (Optional) C++ Integration
#### Setup
To use a C++ inplementation of a Connect Four board with the server, put a board.cc file into the server/cpplib/ directory. Alter the board.hpp function with forward declarations for helper functions if necessary (or move those forward declarations in board.cc).

#### Building
In Cargo.toml uncomment the build-dependencies section. Change build.rs.txt to build.rs, and uncomment / add compilation flags to the method chain as needed for your system.

Now to have the server use the C++ integration, activate the `cppintegration` feature (include it in the default feature list) in Cargo.toml, and run the server.

### (Optional) TLS Websocket Connection
This feature is untested; we never had time to test with actual certificates. We added this feature to be used with our AWS server, because GitHub pages requires HTTP*S* connections. Theoretically this feature would let the GitHub pages client connect to the AWS server.

#### Setup
Get a certificate and key from a certificate authority or generate your own. They must be in .pem format. Take note of the paths to the certificate and key files.

#### Building
Activate the `use-certificate` feature in Cargo.toml by including it in the default feature list, and run the server. Usage: `cargo run <address> --certificate <path to certificate> --key <path to key> --release`