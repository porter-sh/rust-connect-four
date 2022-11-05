# rust-connect-four
## Group
### Name
Iron Oxide
### Memebers
Porter Shawver (porters2)
Alex Broihier (adb12)
## Project
The goal of this project is to make a connect-four game in Rust. We want to have local multiplayer, multiplayer over the internet, and singleplayer against an AI. All of this underneath a pretty UI. This will allow us to learn about front and back end web dev in Rust (yew), machine learning in rust, a connect four algorithm, and we anticipate we will need multithreading either for managing inter-user communication, training of the AI, or both.
### Technical Overview
High level: we are going to make a yew web app that can play connect four locally and communicate with a server to play online.
Slightly lower level: local play is fairly basic with yew components. We can use routing to make the game mode appear in the URL, and communicate state to the board yew component. It seems like websockets are best suited for server communication, and we have found that it works with `gloo-web` on the client side, and `websocket` on the server side. Writing changes in state to the server should be farily straightforward, but reading from the server will require a separate thread to constantly listen for changes. We are thinking that we can have both clients verify if moves are valid or if the game is won as an attempt to prevent the client from cheating. For the AI, we know that connect four is a "solved game," in that there is always a correct move to be made. We are hoping to try to implement an algorithm that finds the best move more efficiently than testing all the future possibilities. Hopefully if we get this working, we can use it to train a neural network just for the experience of doing that. Then we could also train the NN to different amounts so that the player can select the difficulty of the AI.
#### Checkpoint 1 (11/5)
 - Full website layout with routing and and UI.
 - Directive on-screen messages, like "red wins", or "yellow's turn".
 - Game mechanics (placing pieces, stop once someone wins, etc.).
 - Undo button to revert the board state (even to the start of the game).
#### Checkpoint 2 (11/30)
Almost done:
 - Online Multiplayer
 - AI that *at least* works. Ideally we would have our perfect AI mostly done.
 - AI plays in the background against itself on the home screen.
#### End (12/5)
Finishing touches and polish. Maybe Implement some random ideas from below.

### Random Ideas
 - Let the player select which column to drop a disk into by entering a number.
 - Survival mode !!!
 - Undo move option (would need to store move history)
