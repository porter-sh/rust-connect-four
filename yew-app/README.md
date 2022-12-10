# Yew-App
Yew based front end web client that provides user interface, local game modes like local multiplayer and AI, and communicates with the server for online play.

## Run Locally
Run `trunk serve --release` to serve the app on port 8080. Go to `localhost:8080` to view the app. (See RUN.md for how to install trunk.)

## Implementation
### Board
The meat of the app is contained in the board component, which always renders. It is responsible for rendering each column (as a separate compenenet) as well as the buttons and text beneath the baord. Notably, it has a mutable reference-counted reference to `board_state`, which contains all the information about the state of the current game. When the route changes, the board is responsible for updating the `board_state` properly.

#### Column
Each column is a separate component that is responsible for rendering its own game pieces. It puts the clickable buttons on top of itself when it is the current player's turn and not already full. Also, each column has access to a keyboard listener for taking user input from the keyboard. Whenever a column is clicked, it tells the `board_state` to update (which then requests a move from the second player extension), then sends a message to the board to rerender. The entire board needs to rerender mostly necessary for the buttons and text under the board.

### Board State
The board state is a struct that contains all the information about the current game. It encapsulates most of its logic, which makes iteracting with it very simple. Most notable are the `disks` and `second_player_extension` fields. `disks` handles the bitwise representation of the disks in the board, and `second_player_extension` is a blanket representation for the second player.

### Disks
Disks is a struct that handles the bitwise representation of the disks in the board, with a convenient interface to do all the bitwise operations. It uses a total of 17 bytes to represent the board state. It does this by storing one 7x7 grid of bits representing where the current players disks are, then a second 7x7 grid of bits representing where *all* of the disks in the board are. Each grid only needs to be 49 bits, but its easier to just store them in a u64. In both grids, the top row is always empty (because a connect four board is only 7x6) which is useful for a couple of functions.

### Second Player Extention
SecondPlayerExention is a blanket representation for the second player. When the game mode is local multiplayer, SPE dosen't do anything, because the second player is human and needs no representation. Otherwise, the second player is contained in this extension as either an AI or a server connection. This means that from the persepctive of the `board_state`, the the second player is always the same; it always simply requests a move from the second player. This extension is responsible for requesting a move from whatever the second player is (AI or person on the other end of the server connection), and then calling back to the board with the second players move. This means that the first player simply cannot move until the second player move is called back. 

### Router
The router reads the page route (URL), and decides if it needs to render something on top of the board. It is responsible for rendering the menu where the player chooses what game mode to play. 

Based on the route, the switch will supply one of the "pages," as an overlay.

The buttons underneath the board are *not* rendered by the router, but by the board component.
